# TaniTrust DApp
**TaniTrust** — *Blockchain-Based Decentralized Agricultural Escrow and Supply Chain System*

## Project Description

### 1. Background
Agriculture is a vital economic sector in many developing regions, such as the Dieng Plateau in Indonesia, which serves as a major hub for potato production. Despite being the primary producers, small-scale farmers often operate in a fragmented ecosystem. They rely on traditional, word-of-mouth agreements and paper-based tracking, which separates them from the modern digital financial system. In this traditional supply chain, farmers, buyers, and logistics providers operate in a low-trust environment where the farmer usually bears the highest risk.

### 2. The Problem
Farmers face three critical challenges that hinder their economic growth:
* **Lack of Price Transparency & Middlemen Dominance:** Farmers often sell to local intermediaries (*tengkulak*) who dictate prices. Without a transparent record of sales, farmers have little bargaining power and lose a significant portion of their profit to these middlemen.
* **Payment Insecurity:** In many cases, farmers send their harvest but have to wait days or even weeks to receive payment. There is no guarantee that the buyer will pay the full amount or that the funds will arrive on time, creating a "trust gap" that leads to financial instability.
* **Manual & Error-Prone Logistics Splits:** Logistics is a crucial part of the agricultural chain. However, managing payments for transporters is currently a manual process. This often leads to disputes over fees or delays in paying the drivers who are essential for moving the goods from the mountains to the city.

### 3. Our Solution: TaniTrust
**TaniTrust** addresses these issues by moving the agricultural escrow and distribution logic onto the **Stellar Blockchain**. By utilizing Soroban Smart Contracts, we transform a high-risk manual process into a secure, automated, and trustless workflow:

* **Smart Escrow Protection:** When a buyer purchases a harvest batch, TaniTrust acts as a "Digital Notary." The funds are automatically pulled and locked in the smart contract. The farmer is guaranteed that the money exists, and the buyer is guaranteed that the money will only be released once the delivery is confirmed.
* **Automated Split-Payment Logic:** We eliminate the need for manual distribution. The contract is hardcoded with a fair distribution model (90% to the Farmer and 10% to Logistics). This ensures that every stakeholder gets their fair share instantly and accurately the moment the transaction is finalized.
* **Immutable Audit Trail:** Every harvest listed on TaniTrust is recorded permanently on the Stellar ledger. This creates a "Digital Reputation" for farmers, allowing them to build a verified history of their production, which could eventually be used to access formal financial services or credit.
* **Decentralized Governance:** By using blockchain, we remove the need for a central intermediary. The code is the law, ensuring that nobody can manipulate the prices or withhold payments once the contract conditions are met.

## Project Vision
Our vision is to empower farmers by providing them with direct access to a transparent digital economy. We aim to revolutionize the agricultural supply chain by:
* **Eliminating Middlemen:** Connecting farmers directly with buyers to ensure higher profit margins for producers.
* **Guaranteeing Payment Security:** Using smart contract escrows to ensure that farmers are paid the moment their work is verified.
* **Automating Fair Distribution:** Implementing automated split payments to pay both farmers and logistics providers fairly and instantly.
* **Building an Immutable Audit Trail:** Creating a permanent record of harvests and transactions to build a "digital reputation" for farmers.

## Key Features
* **Harvest Registration:** Farmers can list their harvest batches with specific weights, pricing, and assigned logistics providers.
* **Secure Escrow System:** When a purchase is made, funds are automatically pulled from the buyer and locked within the smart contract.
* **Automated Split Payment:** Upon delivery confirmation by the buyer, the contract automatically splits the total payment (90% to the Farmer and 10% to the Logistics provider).
* **Real-time Blockchain Events:** The contract emits specific events for every harvest, purchase, and fund release, allowing for easy tracking via block explorers.
* **Role-Based Authorization:** Ensures that only the authorized farmer can list a harvest and only the designated buyer can confirm the delivery.

## Deployed Smart Contract Details
> [!IMPORTANT]
> **CONTRACT ID:** `CCGWHJTEBNPKAOYKCF6A254THIC5ANTOUJUF2RON4QV4EWOR5SG2BMMV`  
> **NETWORK:** Stellar Testnet (Soroban)  
> **WASM HASH:** `524321dde5ce4e4a3f3c343f378896ec5e6df64caaeb11f7c06a3a14a6ccb0a2`