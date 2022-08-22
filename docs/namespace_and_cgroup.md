本文档描述了我在zcore的基础上实现namespace（主要是这些）和cgroup的部分的工作

（请不要计较本文档的排版，用vim写md让我非常难受）

1/设计内容

    内核数据结构
        
        管理所有namespace的数据结构

        对于单个namespace的数据结构，由于总共有7种namespace，因此数据结构也有不同，他们会放进task中，当然由于这里面task对zircon的继承，所以在zircon-object里面也会有所改动
    
    在整个内核的初始化阶段，对内核相应数据结构进行创建，以及最初的namespace的创建（比如1号进程会归属于默认的namespace）

    （2022.08.22更改设计，内核没有一个用于统一管理所有namespace的实例了，因为我发现没必要，而且，本身namespace的构建应当是一棵树，就是新建namespace会成为上层的子节点，所以，整个数据结构应该改为，最初的那个init的namespace是整个namespace树的根，而且只初始化这个实例，而其他的则像一颗树一样进行操作）
    
    （2022.08.22增加设计，namespace应当被视为一个文件描述符，好吧，我去看文件系统的实现吧）

    7种namespace的实现，以及进程对相应数据结构的访问的函数等等。

    Mount Namespace

    UTS Namespace

    IPC Namespace

    PID Namespace

    Network Namespace

    User Namespace

    Cgroup Namespace

    由于这些namespace很多都有复制原来的，定一个copy的trait吧。

    下面会分别对这些namespace进行分析记录。

    还有就是对clone函数的改进，namespace应当被视作每个进程的属性之一，因此clone创建的时候，会有相应的flag描述对应的namespace，好在flag的工作已经被完成了。

    具体请见linux-syscall/src/task.rs里面CloneFlags的描述。因此，对于相应的flag，需要创建对应的namespace（可能需要阅读linux源码）

    其他：我真的很想弄那个cfg来进行条件编译，但是，我按照说明去toml文件里面加了，以及在makefile 里面加了东西，但是，并没有成功。唯一成功的办法是，关掉--no-default-feature选项，我也很想知道我该怎么做。
    
    当然即便如此，我也不是那种乱放代码的人，我会在我增加的代码部分，增加 #[cfg(feature = "namespace")]的注释，有懂的人帮忙全局搜索一下代码，改成条件编译吧

2/初始化

由于条件编译，需要在toml文件和zcore的makefile更改条件

3/clone函数改写

clone函数中定义了namespace的几个namespace的结构。而原本的clone函数只是简单粗暴的忽略了这几个flag。

显然，这几种ns的组合非常多，需要各自判断if

另外，fork肯定没法改了，所以fork之后会返回那个pid，根据pid去加东西就好了。

4/Mount Namespace

mount主要是文件系统，一个比较麻烦的点在于，他用的fs是rcore的那个fs，而且是使用调包的方式，这就不太好改了。

mount使用的flag是Cloneflags::NEWNS（据说不用newmount的原因是因为当初Linux设计的时候，没考虑还会有其他的namespace，哈哈哈哈）

chroot操作：因为一开始mount namespace是将原先的那个文件系统全都拷贝一遍的，与其他的namespace不同的是，需要添加一个chroot操作，让他的根目录发生变化。

TODO：union fs的问题（反正rcore-fs那部分我是不打算改了）。这个主要是用于docker的文件和镜像多层堆叠的，但是我只是内核部分实现一下。这我表示不做，但是如果要实现一个docker，那么这是必须的。
