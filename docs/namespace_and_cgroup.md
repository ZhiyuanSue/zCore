本文档描述了我在zcore的基础上实现namespace（主要是这些）和cgroup的部分的工作

（请不要计较本文档的排版，用vim写md让我非常难受）

目标

    7种namespace的实现，以及进程对相应数据结构的访问的函数等等。

    Mount Namespace

    UTS Namespace

    IPC Namespace

    PID Namespace

    Network Namespace

    User Namespace

    Cgroup Namespace

1/设计内容及开发log

内核数据结构
    
    管理所有namespace的数据结构

    对于单个namespace的数据结构，由于总共有7种namespace，因此数据结构也有不同，他们会放进task中，当然由于这里面task对zircon的继承，所以在zircon-object里面也会有所改动

在整个内核的初始化阶段，对内核相应数据结构进行创建，以及最初的namespace的创建（比如1号进程会归属于默认的namespace）

（2022.08.22更改设计）

内核没有一个用于统一管理所有namespace的实例了，因为我发现没必要，而且，本身namespace的构建应当是一棵树，就是新建namespace会成为上层的子节点，所以，整个数据结构应该改为，最初的那个init的namespace是整个namespace树的根，而且只初始化这个实例，而其他的则像一颗树一样进行操作。

后来看到setns函数，认为还是有必要实现一个hash，某个hash值指向一个namespace。

（2022.08.22增加设计）

Linux namespace应当被视为一个文件描述符，好吧，我去看文件系统的实现吧，inode应该是一个泛型的实现，这里可以进行扩展。

但是这样有个问题，linux有万物皆文件的思想，但是zcore不是，他没有把进程视作一种文件，所以‘/proc’你是看不到的，因为压根没有，那么本来应该挂在/proc/pid/ns这里的namespace，更加没法实现。

原本，我认为可以通过建立一个ns文件夹把这些给放进去，但是随之而来的诸多问题让我选择放弃。

然而file里面仍然要求给定路径。

（2022.08.25）放弃上面那个想法吧，改成文件太麻烦了。

（2022.08.26） 继续增加改动，使用KoID唯一标识某个ns（这是为了适应这个内核而做的改动），并且和最初想法一样，使用一个统一的全局管理实例，来进行管理，在这同时使用树的方式进行管理（因为他们天然具有树的结构），另外，在Linux中，每个进程都有统一的

由于这些namespace很多都有复制原来的，定一个copy的trait吧。

下面会分别对这些namespace进行分析记录。

还有就是对clone函数的改进，namespace应当被视作每个进程的属性之一，因此clone创建的时候，会有相应的flag描述对应的namespace，好在flag的工作已经被完成了。

具体请见linux-syscall/src/task.rs里面CloneFlags的描述。因此，对于相应的flag，需要创建对应的namespace（可能需要阅读linux源码）

其他：我真的很想弄那个cfg来进行条件编译，但是，我按照说明去toml文件里面加了，以及在makefile 里面加了东西，但是，并没有成功。唯一成功的办法是，关掉--no-default-feature选项，我也很想知道我该怎么做。

当然即便如此，我也不是那种乱放代码的人，我会在我增加的代码部分，增加 #[cfg(feature = "namespace")]的注释，有懂的人帮忙全局搜索一下代码，改成条件编译吧

（2022.08.24）听从杨德瑞大佬的建议，大概是使用了什么什么cfg的模块然后嵌套在里面所以如果外面的那个cfg不给过，然后里面的也就没了，我的意见是，不管这玩意了，先把功能跑通再说吧。

2/初始化

由于条件编译，需要在toml文件和zcore的makefile更改条件（搞不定，不搞了，到时候问别人吧）

初始化一个init的namespace，另外如上面所述，弄一个hash用于id和namespace的对应用于查找。（由于setns函数的语义，显然是全局可见所有内容的）

3/clone函数改写

clone函数中定义了namespace的几个namespace的结构。而原本的clone函数只是简单粗暴的忽略了这几个flag。

显然，这几种ns的组合非常多，需要各自判断if

另外，fork肯定没法改了，所以fork之后会返回那个pid，根据pid去加东西就好了。

4/类的继承

上面这些不同的namespace应当具有一个公共的父类，用于ns树的建立，hash的索引等等。因此需要在这里实现一个继承

5/Mount Namespace

mount主要是文件系统，一个比较麻烦的点在于，他用的fs是rcore的那个fs，而且是使用调包的方式，这就不太好改了。

mount使用的flag是Cloneflags::NEWNS（据说不用newmount的原因是因为当初Linux设计的时候，没考虑还会有其他的namespace，哈哈哈哈）

chroot操作：因为一开始mount namespace是将原先的那个文件系统全都拷贝一遍的，与其他的namespace不同的是，需要添加一个chroot操作，让他的根目录发生变化。

6/Pid Namespace

Pid命名空间和上面有所不同，父空间中能看到子空间的进程，而且他们在不同的空间中存在不同的pid编号。在zcore中原有的设计中已经存在了一个KoID，我认为这无需修改（只不过一些指向这个KoID的操作也许会发生变化）。不同的是，一个进程需要维护在父命名空间和子命名空间（如果有祖父，那也要继续嵌套下去）中的pid映射。

所以在Pid Namespace中不能那么简单的就干下去了。但是如果像Linux那样直接指针指来指去，也很讨厌。rust不能这么干

（2022.08.25）继续更改设计，KoID用于全局标识一个进程的唯一符号——这也没法改呀，他child的列表就是KoID到子进程的hash，改一改，牵一发动全身。所以这样，在Pid Namespace中维护每个进程在该namespace中的一个KoID到namespace下的pid的映射。创建的时候需要向上递归的在ns中增加

7/UTS Namespace

当上面的框架都已经差不多了，这个UTS命名空间还有什么难度啊，UTS主要是记录了内核版本啦，主机名称啦这种东西，框架就上面那个框架，实现我都不想写。难道还有吉祥物认为这种东西很难么？不会吧不会吧。

8/CGroup和rlimit

在zcore，已经整合进去了一部分rlimit的东西——但是貌似syscall::setrlimit和getrlimit并没有实现进去，我也不知道发生了什么。

rlimit是用于限制一个进程的资源使用，而cgroup则是用于限制某个进程组，就是一组进程的资源使用限制的。

然而我实际看了网上的一些对比，发现差别还是蛮大的，比如对cpu限制吧，基于rlimit的ulimit只有对cpu使用时长进行限制，而cgroup还真的限制了CPU使用率。我对于这部分改写的观点是，这部分还是基于已经有的rlimit去做做看吧。

9/TODO

测试样例的问题，这真是让人无奈。/proc没了，依赖于每个proc的ns又该怎么办？(反正这个项目我也只能把功能尽量做全了，实现肯定只能因地制宜了)

（2022.08.24）开会的时候听取了一下杨德瑞大佬的建议，反正还是得自己尝试哪些能用呗。再说吧

union fs的问题（反正rcore-fs那部分我是不打算改了）。这个主要是用于docker的文件和镜像多层堆叠的，但是我只是内核部分实现一下。这我表示不做，但是如果要实现一个docker，那么这是必须的。

cgroup里面会控制某一组资源的使用，但是当多核情况下，这个资源限制怎么弄，我非常的迷惑。目前还在看linux的内核。

（2022.08.24）开会的时候商量了一下，很多地方写的稀烂的代码，我实在不想搞。主要是多核的通信，这还得特意去设计一个算法，太难为我了。

mount namespace中shared subtree，这里我给个知乎链接：https://zhuanlan.zhihu.com/p/166393945 由于存在shared属性和同样的传播属性问题，算了我也讲不清楚，总之，实现我不实现了，但数据结构会尽量设计进去。