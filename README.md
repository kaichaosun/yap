# Yap

Short for Yet Another Parachain.

**Note:**
This is an experiment and expect fail in the end. I only commit a few hours per week to this project.

## Overview

It's known that building a parachain is hard, from dev to market. This repository is aimed to show people how to launch a "profitible" parachain without issue a new token. Building it in the public means,

- every line of code is open sourced
- decisions and thought process are listed
- feedbacks from others will be considered
- metrics like user growth, revenue, etc. will be shared

## Day 0

What to build?

As blockchain is mostly useful for transactional systems, so I need to first find what's is valuable enough to setup such a transactional system. 
- money is valuable in general sense, but too much money printed in paper or crypto way as of now
- people is valuable, find the correct people can be tough for a lot of cases, like business partners, co-workers, etc.
- knowledge is valuable, in words, audio, video, music, art, etc. We have too much information in this digital-verse, filter the right one is hard.
- health is valueable, but people offen ignore it before getting sick. It'll be helpful to ask your family and friends to supervise your excersise, eat healthy, etc.
- kindness is especially valueable in group and community, but it's hard to measure and reward.

I will start from *people*, the first job for this parachain is to help people to find the right people.

## Day 1

To build a parachain, I learned and refreshed some knowledge inclueds,
- Polkadot wiki, [Parachain Development](https://wiki.polkadot.network/docs/build-pdk)
- Substrate tutorial, [Build a parachain](https://docs.substrate.io/tutorials/build-a-parachain/)
- [Rococo Network](https://substrate.io/developers/rococo-network/)

From the tutorial, I found the parachain template, latest version is polkadot-v0.9.40 at the time of writing. So I cloned the repo, renamed to `parachain`, and put it in yap repo,

```shell
git clone -b polkadot-v0.9.40 --depth 1 https://github.com/substrate-developer-hub/substrate-parachain-template
mv substrate-parachain-template parachain
```

To ensure the code is working, I then ran `cargo test` in parachain folder, and all the tests past. After some code cleanup, I'm happy to wrap the day!

## Day 2

Today comes to the business, I need to coding a pallet which can `help people find the right people`, which will likely become a blockchain based referral program later. So this pallet is named `pallet-referral`. The business logic is similar to pallet-assets or pallet-nfts in Substrate, includes,

- create a campaign, which can be deleted or closed later, the campaign has metadata like title, description, company intro, requirements, etc. And optional reward rules for referrals.
- an account generate a referal link for the campaign, which can be shared with others on any platform like twitter, wechat, etc
- candidate apply for the campaign with the referal link by submitting the required information like resume, github handle, linkedin, etc.
- the company process the application by collaborating with the candidate, mostly offline. If accept, the candidate will be hired and the referrer will be rewarded with tokens, stablecoins, or NFTs by following the predefined rules. Otherwise, failed silently.

With these rough ideas in mind, I followed the template pallet, and created the referral pallet. I only create the `Campaigns` storage and `create_campaign` didspatchable so far. Since start a dev network for parachain test is likely tedious, I ran unit tests (`cargo test -p pallet-referral`) to ensure new logic works.