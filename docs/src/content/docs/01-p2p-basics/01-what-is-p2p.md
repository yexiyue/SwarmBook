---
title: 什么是 P2P
description: 一场始于宿舍、终于星辰的去中心化远征
tableOfContents:
  minHeadingLevel: 2
  maxHeadingLevel: 4
---

## 故事的开始：一个大学生的叛逆

1999 年夏末，美国波士顿的一间大学宿舍里，空调嗡嗡作响，18 岁的肖恩·范宁（[Shawn Fanning](https://zh.wikipedia.org/wiki/Shawn_Fanning)）正盯着屏幕上又一次失败的 MP3 下载进度条发呆。他不是第一个为这事烦恼的人，但他决定做点不一样的事。

几天后，一个名为 [Napster](https://zh.wikipedia.org/wiki/Napster) 的程序悄然上线。它没有华丽界面，没有广告，只有一个简单的搜索框和文件列表。但它的背后藏着一个颠覆性的想法：既然每个人都想要音乐，那每个人也都能提供音乐。用户不再从某个遥远的服务器下载歌曲，而是直接从其他正在使用 Napster 的人那里获取。你下载的同时，也在上传；你索取的同时，也在给予。网络的价值，随着用户数量的增长而指数级上升。

短短一年，Napster 用户突破 8000 万。大学生们用它交换 Radiohead 的新专辑，程序员分享 Linux 发行版，地下乐队上传自己的 demo。互联网第一次展现出一种全新的可能性：无需中心，也能协作。这就是 P2P（Peer-to-Peer，点对点）技术第一次震撼世界。

## 唱片业的反击与 Napster 的陨落

但风暴很快降临。2001 年，重金属乐队 Metallica 的鼓手 [Lars Ulrich](https://zh.wikipedia.org/wiki/Lars_Ulrich) 走进法院，手中抱着一摞打印出来的用户名单——那是 Napster 上分享他们歌曲的用户。唱片公司紧随其后，控诉 Napster 助长盗版。法院最终裁定：关闭 Napster 的中央服务器。

命令下达的那一刻，整个网络瞬间瘫痪。数千万用户发现，搜索框再也无法返回任何结果。原来，Napster 虽然传输是点对点的，但"谁有什么文件"这个目录，仍依赖一台位于加州的服务器。邮局一关，信就送不出去了。

Napster 死了，但它播下的种子已经生根发芽。全世界的程序员都在思考同一个问题：如何构建一个真正去中心化的网络，让任何人都无法关闭它？

## 地下世界的进化：从 Gnutella 到 BitTorrent

就在 Napster 被关停的同一年，一个匿名开发者在互联网角落发布了一个新程序，取名 [Gnutella](https://zh.wikipedia.org/wiki/Gnutella)——由"GNU"（自由软件运动）和"Nutella"（人人可分享的巧克力酱）拼合而成。它的宣言简单而激进：我们不需要任何人。

Gnutella 彻底抛弃了中心服务器。每个用户启动后，会向已知节点广播查询："谁有《Bohemian Rhapsody》？"如果对方没有，就继续转发给它的邻居，像涟漪一样在网络中扩散。这种"泛洪式"搜索效率低下，常导致网络拥堵，但它证明了一件事：一个完全去中心化的网络，是可以独立存在的。

人们开始意识到，真正的自由，必须彻底摆脱对任何单一节点的依赖。

但效率问题始终如影随形。直到 2001 年底，一位名叫 [布拉姆·科恩（Bram Cohen）](https://zh.wikipedia.org/wiki/Bram_Cohen) 的程序员在旧金山的一场技术会议上，平静地宣布："我写了一个新协议，叫 [BitTorrent](https://zh.wikipedia.org/wiki/BitTorrent)。"

与 Gnutella 的理想主义不同，BitTorrent 充满工程智慧。它把文件切成小块，允许用户同时从多个来源下载不同部分。更重要的是，它引入了一套精巧的激励机制：客户端会优先上传数据给那些正在上传给你的人，并优先请求最稀有的块，防止某些片段因无人持有而永久丢失。

结果出人意料：热门资源下载飞快，因为每个下载者都在成为上传者。BitTorrent 迅速成为全球事实标准——不仅被用于盗版，也被 [Ubuntu](https://ubuntu.com/)、[Debian](https://www.debian.org/)、[Blender](https://www.blender.org/) 等开源项目官方采用。据统计，高峰时期 BitTorrent 流量占据了全球互联网流量的三分之一。它证明，去中心化不必牺牲效率，甚至可以比中心化更高效。

## 寻找的艺术：Kademlia 与 DHT

可新的难题接踵而至：如果没有 Tracker（追踪器）告诉用户彼此的地址，他们如何在茫茫人海中找到对方？

答案在 2002 年揭晓。两位科学家 [Petar Maymounkov](https://zh.wikipedia.org/wiki/Petar_Maymounkov) 和 [David Mazières](https://zh.wikipedia.org/wiki/David_Mazi%C3%A8res) 发表了一篇论文，提出一种名为 [Kademlia](https://pdos.csail.mit.edu/~petar/papers/maymounkov-kademlia-lncs.pdf) 的分布式哈希表（DHT）算法。

它像一张动态更新的社交地图：每个节点只需记住一小群"朋友"，却能通过"朋友的朋友"快速定位全球任意资源。搜索不再是盲目的泛洪，而是有方向的导航。就像你不需要认识全世界的人，只需要认识几个朋友，通过"[六度分隔](https://zh.wikipedia.org/wiki/Six_degrees_of_separation)"就能联系到任何人。

2005 年，BitTorrent 整合了 Kademlia（称为 [Mainline DHT](https://wiki.theory.org/BitTorrentSpecification#Distributed_Hash_Table_.28DHT.29)），彻底摆脱对 Tracker 的依赖。从此，即使所有中心服务器消失，网络依然能自我发现、自我连接。P2P 真正走向了自治。

## 自由的代价：Freenet 与 I2P

与此同时，另一群人思考着更深层的问题：如果网络可以共享文件，能否也共享自由？

2000 年，[Ian Clarke](https://en.wikipedia.org/wiki/Ian_Clarke_(computer_scientist)) 发布 [Freenet](https://zh.wikipedia.org/wiki/Freenet)，一个旨在抵抗审查的匿名 P2P 网络。用户可以匿名发布文章、图片或软件，系统自动加密并分散存储，政府或 ISP 无法追踪源头。

几年后，[I2P（Invisible Internet Project）](https://geti2p.net/) 更进一步，构建了一整套加密覆盖网络，让通信本身不可见。P2P 不再只是工具，而成为数字抵抗的基础设施。

## 货币的革命：比特币与区块链

但真正的范式转移，发生在 2008 年 10 月 31 日。那天，一个化名**[中本聪（Satoshi Nakamoto）](https://zh.wikipedia.org/wiki/Satoshi_Nakamoto)** 的人在密码学邮件列表中发布了一篇白皮书：《[比特币：一种点对点的电子现金系统](https://bitcoin.org/bitcoin.pdf)》。

他没有发明新密码学，也没有创造新网络协议。但他做了一件前所未有的事：将 P2P 网络作为信任的载体。

在比特币中，没有银行，没有央行，没有中介。每笔交易由全网节点广播、验证，并通过工作量证明达成共识。账本（区块链）由所有参与者共同维护，不可篡改，无需许可。

2009 年 1 月 3 日，中本聪挖出了比特币的第一个区块，在里面留下了一句话："The Times 03/Jan/2009 Chancellor on brink of second bailout for banks"（泰晤士报 2009 年 1 月 3 日 财政大臣即将对银行进行第二轮救助）。这句话是对传统金融体系的嘲讽，也是对 P2P 理想的宣言：我们不需要中心化的机构来管理我们的钱。

P2P 不再只是传递文件，而是传递价值、建立规则、铸造制度。去中心化，从此有了经济意义。

比特币之后，[以太坊](https://ethereum.org/)、[Solana](https://solana.com/)、[Polkadot](https://polkadot.network/) 等区块链项目相继出现，它们都建立在 P2P 网络之上。区块链让 P2P 技术从极客的玩具变成了万亿美元的产业。

## 超越文件分享：IPFS 的野心

受此启发，2015 年，[协议实验室（Protocol Labs）](https://protocol.ai/) 推出 [IPFS（InterPlanetary File System，星际文件系统）](https://ipfs.tech/)，目标直指 HTTP 协议本身。

HTTP 基于地址寻址——链接指向服务器位置，一旦服务器关闭，内容便消失于"404"。IPFS 则采用内容寻址：每个文件生成唯一哈希 ID，任何拥有该文件的节点都可提供服务。

IPFS 整合了 BitTorrent 的分块传输、Kademlia 的分布式路由，还加入了 Git 的版本控制思想。它试图构建一个永久存续、抗审查、用户主权的下一代互联网。

## libp2p：集大成者

二十多年的 P2P 发展史，留下了无数宝贵的经验和教训。但每个项目都在重复造轮子：Napster 写一套网络代码，BitTorrent 写一套，IPFS 又写一套……

2015 年，IPFS 团队决定把他们积累的 P2P 网络代码抽取出来，做成一个独立的、模块化的网络库。这就是 [libp2p](https://libp2p.io/)。

libp2p 不是一个应用，而是一个工具箱。它把 P2P 网络中的各种问题——节点发现、安全传输、NAT 穿透、消息路由——都抽象成可插拔的模块。你可以根据自己的需求，像搭积木一样组合这些模块。

今天，libp2p 已经被众多项目采用：IPFS、[Filecoin](https://filecoin.io/)、[以太坊 2.0](https://ethereum.org/en/eth2/)、[Polkadot](https://wiki.polkadot.network/docs/learn-libp2p)……它成为了 P2P 网络开发的事实标准。

而这，就是我们这个教程要学习的东西。

## 为什么 P2P 重要？

从 Napster 的宿舍代码，到比特币的创世区块，再到 IPFS 的星际愿景，P2P 走过了二十五年。它始于一个少年对慢速下载的不满，却最终演变为一场关于权力、自由与协作的宏大实验。

它告诉我们：

- **去中心化**：网络不必由巨头掌控。Napster 可以被关闭，但 BitTorrent 网络至今仍在运行。
- **抗审查**：信息不必经手中心。没有中心服务器可以被攻击或被命令关闭。
- **可扩展**：传统服务器用户越多压力越大，P2P 网络用户越多反而越强。
- **隐私保护**：数据直接在用户之间传输，不经过第三方服务器。
- **资源利用**：利用参与者的闲置带宽和存储，而不是依赖昂贵的中心化基础设施。
- **信任重构**：信任不必依赖权威，秩序可以在没有指挥的情况下自生。

## 我们要做什么？

在这个教程中，我们将用 Rust 和 libp2p 构建一个 P2P 协作编辑后端——类似 Google Docs 的实时协作功能，但不需要 Google 的服务器。

想象一下：你和同事一起编辑文档，数据直接在你们的电脑之间同步，不经过任何第三方服务器。没有人能偷看你们的内容，没有人能关闭这个服务。

这就是 P2P 的魅力，也是我们要实现的目标。

## 小结

从 1999 年 Napster 的横空出世，到今天遍布全球的区块链网络，P2P 技术走过了二十五年的历程。这段历史告诉我们：

- P2P 是节点直接通信的网络架构，没有中心服务器
- 从 Napster 到 BitTorrent 到比特币，P2P 技术在对抗与进化中不断成熟
- libp2p 集成了二十五年的经验，成为 P2P 开发的标准工具箱

而这，就是 P2P 的真正含义——Peer to Peer，人与人的直接连接，无需许可，无需中介，无需恐惧。

下一章，我们来看 libp2p 的整体架构设计，了解它是如何把这些复杂的问题变成简单的模块的。
