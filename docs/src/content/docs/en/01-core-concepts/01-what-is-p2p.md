---
title: What is P2P
description: A decentralization odyssey from dorm rooms to the stars
tableOfContents:
  minHeadingLevel: 2
  maxHeadingLevel: 5
---

## Napster: Where It All Began

Late summer 1999. In a Boston college dorm room, the air conditioner humming, 18-year-old [Shawn Fanning](https://en.wikipedia.org/wiki/Shawn_Fanning) stared at yet another failed MP3 download progress bar. He wasn't the first to be frustrated by this, but he decided to do something different.

Days later, a program called [Napster](https://en.wikipedia.org/wiki/Napster) quietly went online. No fancy interface, no ads—just a simple search box and file list. But behind it lay a revolutionary idea: since everyone wants music, everyone can provide music. Users no longer downloaded songs from some distant server; they got them directly from other Napster users. While downloading, you were also uploading; while taking, you were also giving. The network's value grew exponentially with each new user.

Within a year, Napster had over 80 million users. College students traded Radiohead's latest album, programmers shared Linux distributions, underground bands uploaded their demos. For the first time, the internet revealed a new possibility: collaboration without a center. This was P2P (Peer-to-Peer) technology shaking the world for the first time.

### The Fall of Napster

But the storm came quickly. In 2001, Metallica's drummer [Lars Ulrich](https://en.wikipedia.org/wiki/Lars_Ulrich) walked into court carrying a stack of printed usernames—people who had shared the band's songs on Napster. Record companies followed, accusing Napster of facilitating piracy. The court's final ruling: shut down Napster's central servers.

The moment the order was executed, the entire network collapsed instantly. Tens of millions of users found that the search box no longer returned any results. It turned out that while Napster's file transfers were peer-to-peer, the directory of "who has what files" still depended on a single server in California. When the post office closes, no letters get delivered.

Napster died, but the seeds it planted had already taken root. Programmers around the world were pondering the same question: How do you build a truly decentralized network that no one can shut down?

### Gnutella and BitTorrent

The same year Napster was shut down, an anonymous developer released a new program in a corner of the internet, naming it [Gnutella](https://en.wikipedia.org/wiki/Gnutella)—a blend of "GNU" (the free software movement) and "Nutella" (chocolate spread everyone can share). Its manifesto was simple and radical: we don't need anyone.

Gnutella completely abandoned central servers. When a user started up, they would broadcast queries to known nodes: "Who has 'Bohemian Rhapsody'?" If the peer didn't have it, the query would be forwarded to its neighbors, spreading through the network like ripples in a pond. This "flooding" search was inefficient and often caused network congestion, but it proved one thing: a fully decentralized network can exist independently.

People began to realize that true freedom required complete independence from any single node.

But efficiency problems persisted. Until late 2001, when a programmer named [Bram Cohen](https://en.wikipedia.org/wiki/Bram_Cohen) calmly announced at a San Francisco tech conference: "I've written a new protocol called [BitTorrent](https://en.wikipedia.org/wiki/BitTorrent)."

Unlike Gnutella's idealism, BitTorrent was full of engineering wisdom. It split files into small pieces, allowing users to download different parts from multiple sources simultaneously. More importantly, it introduced an elegant incentive mechanism: the client prioritizes uploading to peers who are uploading to you, and preferentially requests the rarest pieces to prevent permanent loss of fragments.

The result was surprising: popular resources downloaded blazingly fast because every downloader became an uploader. BitTorrent quickly became the global de facto standard—used not only for piracy but officially adopted by open-source projects like [Ubuntu](https://ubuntu.com/), [Debian](https://www.debian.org/), and [Blender](https://www.blender.org/). At its peak, BitTorrent traffic accounted for one-third of all internet traffic. It proved that decentralization doesn't have to sacrifice efficiency—it can even be more efficient than centralization.

### Kademlia and DHT

But new challenges emerged: without a Tracker telling users each other's addresses, how would they find each other in the vast crowd?

The answer came in 2002. Two scientists, [Petar Maymounkov](https://scholar.google.com/citations?user=wyMVuCsAAAAJ) and [David Mazières](https://en.wikipedia.org/wiki/David_Mazi%C3%A8res), published a paper proposing a distributed hash table (DHT) algorithm called [Kademlia](https://pdos.csail.mit.edu/~petar/papers/maymounkov-kademlia-lncs.pdf).

It works like a dynamically updated social map: each node only needs to remember a small group of "friends," yet can quickly locate any resource globally through "friends of friends." Searches were no longer blind flooding but directed navigation. Just as you don't need to know everyone in the world—you just need to know a few friends, and through "[six degrees of separation](https://en.wikipedia.org/wiki/Six_degrees_of_separation)," you can reach anyone.

In 2005, BitTorrent integrated Kademlia (called [Mainline DHT](https://wiki.theory.org/BitTorrentSpecification#Distributed_Hash_Table_.28DHT.29)), completely freeing itself from Tracker dependence. From then on, even if all central servers disappeared, the network could still discover and connect to itself. P2P had truly become self-governing.

### Freenet and I2P

Meanwhile, another group was thinking about deeper questions: if networks can share files, can they also share freedom?

In 2000, [Ian Clarke](https://en.wikipedia.org/wiki/Ian_Clarke_(computer_scientist)) released [Freenet](https://en.wikipedia.org/wiki/Freenet), an anonymous P2P network designed to resist censorship. Users could anonymously publish articles, images, or software, with the system automatically encrypting and distributing storage, making it impossible for governments or ISPs to track the source.

Years later, [I2P (Invisible Internet Project)](https://geti2p.net/) went further, building a complete encrypted overlay network that made the communication itself invisible. P2P was no longer just a tool—it became infrastructure for digital resistance.

### Bitcoin and Blockchain

But the true paradigm shift happened on October 31, 2008. That day, someone using the pseudonym **[Satoshi Nakamoto](https://en.wikipedia.org/wiki/Satoshi_Nakamoto)** posted a white paper to a cryptography mailing list: "[Bitcoin: A Peer-to-Peer Electronic Cash System](https://bitcoin.org/bitcoin.pdf)."

He didn't invent new cryptography or create new network protocols. But he did something unprecedented: he made the P2P network a carrier of trust.

In Bitcoin, there are no banks, no central banks, no intermediaries. Every transaction is broadcast, verified by nodes across the network, and consensus is reached through proof of work. The ledger (blockchain) is maintained by all participants, immutable and permissionless.

On January 3, 2009, Satoshi mined Bitcoin's first block and left a message inside: "The Times 03/Jan/2009 Chancellor on brink of second bailout for banks." This was both a mockery of the traditional financial system and a declaration of P2P ideals: we don't need centralized institutions to manage our money.

P2P was no longer just about transferring files—it was about transferring value, establishing rules, and forging institutions. Decentralization now had economic meaning.

After Bitcoin, [Ethereum](https://ethereum.org/), [Solana](https://solana.com/), [Polkadot](https://polkadot.network/), and other blockchain projects emerged, all built on P2P networks. Blockchain transformed P2P technology from a geek's toy into a trillion-dollar industry.

### IPFS

Inspired by this, in 2015, [Protocol Labs](https://protocol.ai/) launched [IPFS (InterPlanetary File System)](https://ipfs.tech/), targeting the HTTP protocol itself.

HTTP is location-addressed—links point to server locations, and once a server goes down, content disappears into "404." IPFS uses content-addressing instead: each file generates a unique hash ID, and any node possessing that file can serve it.

IPFS integrates BitTorrent's chunked transfer, Kademlia's distributed routing, and Git's version control concepts. It attempts to build a next-generation internet that's permanent, censorship-resistant, and user-sovereign.

### libp2p

Over twenty years of P2P history left countless valuable lessons. But every project kept reinventing the wheel: Napster wrote its networking code, BitTorrent wrote its own, IPFS wrote yet another...

In 2015, the IPFS team decided to extract their accumulated P2P networking code into a standalone, modular networking library. This became [libp2p](https://libp2p.io/).

libp2p isn't an application—it's a toolbox. It abstracts the various challenges in P2P networking—peer discovery, secure transport, NAT traversal, message routing—into pluggable modules. You can combine these modules like building blocks according to your needs.

Today, libp2p has been adopted by numerous projects: IPFS, [Filecoin](https://filecoin.io/), [Ethereum 2.0](https://ethereum.org/en/eth2/), [Polkadot](https://wiki.polkadot.network/docs/learn-libp2p)... It has become the de facto standard for P2P network development.

And this is what we're going to learn in this tutorial.

### Why P2P Matters

From Napster's dorm room code to Bitcoin's genesis block to IPFS's interplanetary vision, P2P has journeyed twenty-five years. It began with a teenager's frustration over slow downloads and evolved into a grand experiment about power, freedom, and collaboration.

It teaches us:

- **Decentralization**: Networks don't have to be controlled by giants. Napster could be shut down, but the BitTorrent network is still running today.
- **Censorship Resistance**: Information doesn't have to go through a center. There's no central server to attack or order to shut down.
- **Scalability**: Traditional servers struggle under more users; P2P networks grow stronger with more participants.
- **Privacy Protection**: Data travels directly between users, bypassing third-party servers.
- **Resource Efficiency**: Utilizing participants' idle bandwidth and storage instead of relying on expensive centralized infrastructure.
- **Trust Reimagined**: Trust doesn't have to rely on authority; order can emerge without command.

## What Are We Building?

In this tutorial, we'll use Rust and libp2p to build a P2P collaborative editing backend—similar to Google Docs' real-time collaboration features, but without Google's servers.

Imagine: you and your colleagues editing a document together, with data syncing directly between your computers, not passing through any third-party server. No one can spy on your content, no one can shut down this service.

This is the magic of P2P, and this is our goal.

## Summary

From Napster's dramatic debut in 1999 to today's global blockchain networks, P2P technology has journeyed twenty-five years. This history tells us:

- P2P is a network architecture where nodes communicate directly, without central servers
- From Napster to BitTorrent to Bitcoin, P2P technology has matured through conflict and evolution
- libp2p integrates twenty-five years of experience, becoming the standard toolbox for P2P development

And this is the true meaning of P2P—Peer to Peer, direct connection between people, without permission, without intermediaries, without fear.

Next chapter, we'll look at libp2p's overall architecture design and see how it transforms these complex problems into simple modules.
