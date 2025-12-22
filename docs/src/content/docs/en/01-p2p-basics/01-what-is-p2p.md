---
title: What is P2P
description: A decentralized expedition from dorm room to the stars
tableOfContents:
  minHeadingLevel: 2
  maxHeadingLevel: 4
---

## The Beginning: A College Student's Rebellion

In late summer 1999, in a college dorm room in Boston, with the air conditioner humming, 18-year-old [Shawn Fanning](https://en.wikipedia.org/wiki/Shawn_Fanning) stared blankly at yet another failed MP3 download progress bar. He wasn't the first to be frustrated by this, but he decided to do something different.

A few days later, a program called [Napster](https://en.wikipedia.org/wiki/Napster) quietly went online. It had no fancy interface, no ads—just a simple search box and file list. But behind it lay a revolutionary idea: since everyone wants music, everyone can also provide music. Users no longer downloaded songs from some distant server but directly from other Napster users. While downloading, you were also uploading; while taking, you were also giving. The network's value grew exponentially with the number of users.

Within a year, Napster had over 80 million users. College students exchanged Radiohead's new album, programmers shared Linux distributions, underground bands uploaded their demos. For the first time, the internet demonstrated a new possibility: collaboration without a center. This was P2P (Peer-to-Peer) technology's first moment of shocking the world.

## The Record Industry Strikes Back and Napster's Fall

But the storm came quickly. In 2001, Metallica's drummer [Lars Ulrich](https://en.wikipedia.org/wiki/Lars_Ulrich) walked into court carrying a stack of printed usernames—users who had shared their songs on Napster. Record companies followed suit, accusing Napster of facilitating piracy. The court ultimately ruled: shut down Napster's central server.

The moment the order was executed, the entire network collapsed instantly. Tens of millions of users found that the search box no longer returned any results. It turned out that while Napster's file transfers were peer-to-peer, the directory of "who has what files" still depended on a server in California. When the post office closes, letters can't be delivered.

Napster died, but the seeds it planted had already taken root. Programmers worldwide were pondering the same question: how to build a truly decentralized network that no one could shut down?

## Underground Evolution: From Gnutella to BitTorrent

The same year Napster was shut down, an anonymous developer released a new program in a corner of the internet, named [Gnutella](https://en.wikipedia.org/wiki/Gnutella)—a combination of "GNU" (the free software movement) and "Nutella" (chocolate spread everyone can share). Its manifesto was simple and radical: we don't need anyone.

Gnutella completely abandoned central servers. When users started up, they would broadcast queries to known nodes: "Who has 'Bohemian Rhapsody'?" If the recipient didn't have it, they'd forward the query to their neighbors, spreading like ripples across the network. This "flooding" search was inefficient and often caused network congestion, but it proved one thing: a completely decentralized network can exist independently.

People began to realize that true freedom requires complete independence from any single node.

But efficiency problems persisted. Until late 2001, when a programmer named [Bram Cohen](https://en.wikipedia.org/wiki/Bram_Cohen) calmly announced at a tech conference in San Francisco: "I've written a new protocol called [BitTorrent](https://en.wikipedia.org/wiki/BitTorrent)."

Unlike Gnutella's idealism, BitTorrent was full of engineering wisdom. It split files into small pieces, allowing users to download different parts from multiple sources simultaneously. More importantly, it introduced an elegant incentive mechanism: clients would prioritize uploading to those who were uploading to them, and prioritize requesting the rarest pieces to prevent some fragments from being permanently lost.

The result was surprising: popular resources downloaded blazingly fast because every downloader was becoming an uploader. BitTorrent quickly became the global de facto standard—used not only for piracy but also officially adopted by open-source projects like [Ubuntu](https://ubuntu.com/), [Debian](https://www.debian.org/), and [Blender](https://www.blender.org/). At its peak, BitTorrent traffic accounted for one-third of global internet traffic. It proved that decentralization doesn't have to sacrifice efficiency—it can even be more efficient than centralization.

## The Art of Discovery: Kademlia and DHT

But new challenges emerged: without a Tracker telling users each other's addresses, how could they find each other in the vast crowd?

The answer came in 2002. Two scientists, [Petar Maymounkov](https://en.wikipedia.org/wiki/Petar_Maymounkov) and [David Mazières](https://en.wikipedia.org/wiki/David_Mazi%C3%A8res), published a paper proposing a distributed hash table (DHT) algorithm called [Kademlia](https://pdos.csail.mit.edu/~petar/papers/maymounkov-kademlia-lncs.pdf).

It's like a dynamically updated social map: each node only needs to remember a small group of "friends," yet can quickly locate any resource globally through "friends of friends." Search was no longer blind flooding but directed navigation. Just as you don't need to know everyone in the world—you just need to know a few friends, and through "[six degrees of separation](https://en.wikipedia.org/wiki/Six_degrees_of_separation)" you can reach anyone.

In 2005, BitTorrent integrated Kademlia (called [Mainline DHT](https://wiki.theory.org/BitTorrentSpecification#Distributed_Hash_Table_.28DHT.29)), completely freeing itself from Tracker dependency. From then on, even if all central servers disappeared, the network could still self-discover and self-connect. P2P truly became autonomous.

## The Price of Freedom: Freenet and I2P

Meanwhile, another group was thinking about deeper questions: if networks can share files, can they also share freedom?

In 2000, [Ian Clarke](https://en.wikipedia.org/wiki/Ian_Clarke_(computer_scientist)) released [Freenet](https://en.wikipedia.org/wiki/Freenet), an anonymous P2P network designed to resist censorship. Users could anonymously publish articles, images, or software, with the system automatically encrypting and distributing storage—governments or ISPs couldn't trace the source.

A few years later, [I2P (Invisible Internet Project)](https://geti2p.net/) went further, building an entire encrypted overlay network that made communication itself invisible. P2P was no longer just a tool but became infrastructure for digital resistance.

## The Currency Revolution: Bitcoin and Blockchain

But the true paradigm shift happened on October 31, 2008. That day, someone using the pseudonym **[Satoshi Nakamoto](https://en.wikipedia.org/wiki/Satoshi_Nakamoto)** published a white paper on a cryptography mailing list: "[Bitcoin: A Peer-to-Peer Electronic Cash System](https://bitcoin.org/bitcoin.pdf)."

He didn't invent new cryptography or create new network protocols. But he did something unprecedented: using a P2P network as a carrier of trust.

In Bitcoin, there are no banks, no central banks, no intermediaries. Every transaction is broadcast, verified by network nodes, and consensus is reached through proof of work. The ledger (blockchain) is maintained by all participants—immutable and permissionless.

On January 3, 2009, Satoshi mined Bitcoin's first block, leaving a message inside: "The Times 03/Jan/2009 Chancellor on brink of second bailout for banks." This was both a mockery of the traditional financial system and a declaration of P2P ideals: we don't need centralized institutions to manage our money.

P2P was no longer just about transferring files but transferring value, establishing rules, and minting institutions. Decentralization now had economic significance.

After Bitcoin, blockchain projects like [Ethereum](https://ethereum.org/), [Solana](https://solana.com/), and [Polkadot](https://polkadot.network/) emerged, all built on P2P networks. Blockchain transformed P2P technology from a geek's toy into a trillion-dollar industry.

## Beyond File Sharing: IPFS's Ambition

Inspired by this, in 2015, [Protocol Labs](https://protocol.ai/) launched [IPFS (InterPlanetary File System)](https://ipfs.tech/), targeting the HTTP protocol itself.

HTTP is based on location addressing—links point to server locations, and once a server shuts down, content disappears into "404." IPFS uses content addressing: each file generates a unique hash ID, and any node holding that file can serve it.

IPFS integrates BitTorrent's chunked transfers, Kademlia's distributed routing, and adds Git's version control philosophy. It attempts to build a next-generation internet that's permanent, censorship-resistant, and user-sovereign.

## libp2p: The Grand Synthesis

Over twenty years of P2P development left countless valuable experiences and lessons. But every project kept reinventing the wheel: Napster wrote one set of networking code, BitTorrent wrote another, IPFS wrote yet another...

In 2015, the IPFS team decided to extract their accumulated P2P networking code into an independent, modular networking library. This became [libp2p](https://libp2p.io/).

libp2p isn't an application but a toolbox. It abstracts various P2P networking problems—node discovery, secure transport, NAT traversal, message routing—into pluggable modules. You can combine these modules like building blocks according to your needs.

Today, libp2p has been adopted by numerous projects: IPFS, [Filecoin](https://filecoin.io/), [Ethereum 2.0](https://ethereum.org/en/eth2/), [Polkadot](https://wiki.polkadot.network/docs/learn-libp2p)... It has become the de facto standard for P2P network development.

And this is what we'll be learning in this tutorial.

## Why Does P2P Matter?

From Napster's dorm room code to Bitcoin's genesis block to IPFS's interplanetary vision, P2P has traveled twenty-five years. It began with a teenager's frustration with slow downloads but ultimately evolved into a grand experiment about power, freedom, and collaboration.

It tells us:

- **Decentralization**: Networks don't have to be controlled by giants. Napster could be shut down, but the BitTorrent network is still running today.
- **Censorship Resistance**: Information doesn't have to pass through a center. No central server can be attacked or ordered to shut down.
- **Scalability**: Traditional servers face more pressure with more users; P2P networks get stronger with more users.
- **Privacy Protection**: Data transfers directly between users without passing through third-party servers.
- **Resource Utilization**: Leverages participants' idle bandwidth and storage instead of relying on expensive centralized infrastructure.
- **Trust Reconstruction**: Trust doesn't have to depend on authority; order can emerge without command.

## What Will We Build?

In this tutorial, we'll use Rust and libp2p to build a P2P collaborative editing backend—similar to Google Docs' real-time collaboration, but without Google's servers.

Imagine: you and your colleagues editing a document together, with data syncing directly between your computers, not passing through any third-party server. No one can peek at your content, no one can shut down this service.

This is the charm of P2P, and this is our goal.

## Summary

From Napster's explosive debut in 1999 to today's global blockchain networks, P2P technology has traveled twenty-five years. This history tells us:

- P2P is a network architecture where nodes communicate directly, without central servers
- From Napster to BitTorrent to Bitcoin, P2P technology matured through confrontation and evolution
- libp2p integrates twenty-five years of experience, becoming the standard toolbox for P2P development

And this is the true meaning of P2P—Peer to Peer, direct connection between people, without permission, without intermediaries, without fear.

In the next chapter, we'll look at libp2p's overall architecture design and understand how it turns these complex problems into simple modules.
