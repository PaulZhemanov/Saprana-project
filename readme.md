# Introduction

A platform that allows you to raise funds to organize concerts, promotions and other events through NFT. 
Our project is a decentralized platform for organizing concerts, promotions, and other events through the use of NFT technology. We aim to provide users with a convenient marketplace where they can purchase tickets to events and gain access to additional benefits associated with owning NFTs.

## Decentralized Ticket Marketplace with Ongoing NFT Benefits: A Collaborative Vision with Saprana Project

* There should be a marketplace where people can buy tickets. 
* Users need to be able to browse tickets in their wallets
* NFT Hodlers get access to all goodies on site
* These NFT tickets are not a one-off purchase. the story continues after the event is over. Some digital content should be there, access to a gated community, it can be traded even after it’s over. So all that should be designed in collab with Saprana project.

## Project Description

The project is actively under development, and we are excited to present the demo version on the beta-3 testnet by Fuel. We invite all interested users to join our testing environment and explore the capabilities of our platform. Please note that the demo version is for testing purposes only and may have some temporary limitations and restricted content.

## Guide

Start by [installing the Rust toolchain.](https://www.rust-lang.org/tools/install)

Then, [install the Fuel toolchain](https://github.com/FuelLabs/fuelup)

* Build the Contract

From inside the    **Saprana-project/contract** directory, run the following command to build your contract:
```
fuelup default beta-3 && forc build && fuelup default latest
```
Switching to latest toolchain is necessary because the latest version of Sway does not support beta-3.
* Testing Contract

For running tests use **cargo test** in the terminal.
```
cargo test
```
You can also run each of the tests separately and monitor the performance of the contract. Open a file with any test and run it using the command
```
cargo test -- --nocapture  
```




* Video: https://youtu.be/gGHyVmrOEHE
* Presentation: https://docs.google.com/presentation/d/1l1yzOYVauh-61NaQh9iWtSM6dOgTe0Ng3x6M9vMVmEA/edit?usp=drive_link
