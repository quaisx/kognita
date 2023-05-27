#PROJECT
```code
     _  __ ____  _____ _      _  _____  ____ 
    / |/ //  _ \/  __// \  /|/ \/__ __\/  _ \
    |   / | / \|| |  _| |\ ||| |  / \  | / \|
    |   \ | \_/|| |_//| | \||| |  | |  | |-||
    \_|\_\\____/\____\\_/  \|\_/  \_/  \_/ \|                                     
```

# LATEST UPDATE
I am working on branch feat/tokio to migrate to tokio async library.

## Introduction
This is my attempt to write a crypto platform using my previous experience with a variety of crypto projects I have worked on in the past. I have used such programming languages as Python, Go, JavaScript and all these languages have their drawbacks. The biggest three problems I have faced while using these programming languages are:
1. Performance
2. Concurrency
3. Reliability

## Why RUST for a crypto project?
Well, I am not stepping into a phylosophical battle which programming language is the best overall. I am a beliver in choosing the right weapon for the right battle. And given my previous experience with crypto projects I think that Rust is an excellent choice for creating a crypto platform that would allow for blasting fast processing of high volumes of microtransactions. Think IoT. Think DAG. Think Markov Chain Monte Carlo. Cryptography. Multihashes. The list goes on. 


## Expectations and reality
Each node should be able to handle at least 1000+ microtransactions per second. A network of nodes should be able to handle an unlimited number of transactions per second. One of the biggest questions I have not answered yet - how to store such amounts of information about a high volume of transactions? If the network can handle an unlimited number of transactions, how will it store an infinite amount of data? Ambitions should have strong footing in reality. There is a lot of thinking ahead. 

## Current status
As of right now, this is work in progress. The work in experimental and I expect a lot of changes to the codebase in the near future. As of this writing - I am adding boilerplate code to have a basic node start up and exchange randomly generated transactions between nodes. 

## What has been learned so far?
One of the biggest challenges I have faced is the networking and tackling various networking topologies. I need to rethink transaction propogation approach as using libp2p pubsub has been somewhat desastrous. So, the main focus on the initial stages of development will be addressing the issues of connectivity. I need to build a solution that guarantees at least 90-95% success in establishing P2P connections. The rest - should and most likely will go through the relay servers.

### Special note on the use of relay servers
Since the soluton should be fully decentralized there should not be any centrally controlled infrastructure to facilitate the network. Hence, carefully chosen nodes should be elected by the network to provide such functionality in exchange for a reward. Which brings us to two important qeustions I will need to solve and address:
1. Motivation/Reward
2. Rating/Election

## Motivation/Reward
I will need to work out a viable and econimically sound motivation model which will allow all the participants contributing to the network to be compensated and rewarded. How? Gas fees? Is there another solution? Most likely. Especially when dealing with high volumes of transactions the reward mechanism needs to be carefully designed. Need to work on this problem in the background for now and do a lot of reading on crypto economy.

## Ratings/Elections
The network should be self gouverned and the nodes themselves must elect the nodes with the highest ratings to perform the most trustworthy operations, such as transaction validation. This model should be resistant to various exploits to prevent malicious nodes from taking over the network. The rating system should be fair and all nodes should have a chance to participate in activities that allow them to earn fair share of profits while participating on the network. 

## Fairness

## Security

## Scalability

## Marketability

## User Interface

... and many other topics.

# Conclusion
Once again, this is work in progress. I do not expect this platform to be a viable candidate for replacing Ethereum or Bitcoin. That would be too ambisious of me, and, frankly, unachievable with the manpower/hours available to build such a network any time soon. But, as the saying goes - a thousand mile journey starts with a single step. The only goal I have determination to achieve is to become a better Rust architect/developer. 