# **GrofDB** 🌟  
GrofDB is an acronym for low latency key value database. Filesystems are major considerations during system designs of any kind. Proper handling of files and data either through Relational database management systems, NoSQL Databases or even Key-Value stores cannot be overemphasized. GrofDB is a network available key value store database that is built based on the design principles of _Log Structured Merge Trees_ in this project my approach conveys a wholistic and Lean structure. GrofDB is designed to solve some of the issues traditional operating systems face in File systems modules which are high latency and high memory usages and sometimes Runtime complexities.

## **Table of Contents**  
1. [Description](#description)  
2. [Features](#features)  
3. [Prerequisites](#Prerequisites)
3. [Installation](#installation)  
4. [Usage](#usage) 
5. [Make Usage](#make-usage)
6. [Endpoints](#endpoints)
7. [Notes](#notes)

## **Description**  
**GrofDB** is a network available key value store database that is built based on the design principles of _Log Structured Merge Trees_ which can be incorporated in any system to serve as a datastore. GrofDB provides low latency during writes and reads operations on any system it is incorporated on. it also provides a fault tolerant system which helps to preserve data incase of unforseen circumstances or a system crash. GrofDB pays attention to the ACID properties of database engineering to enhance data safety and durability, It also uses algorithms like Quicksort (efficient for sorting large datasets), Binary search and Log Merge trees for internal file operations.

### **Key Highlights:**  
- **Problem it solves**: High Latency and High Memory utilisation issues in mordern Database systems  
- **Where can it be used**: Embedded systems Datastores, Web systems that require any form of Datastore etc.   


## **Features**  
| Feature |  Summary | Status     |  
|-------------|-------------|------------|  
| 🌟 **Write Ahead Log**   |  Takes in data before sent to memtables      |  ✅ |  
| 📝 **In-Mem Disk Memtable**   | Holds writes data request from server      |  ✅ |  
| ⚙️ **Sorted Tables**   | Memtables Flush on threshold limit to disk     |  ✅ |  
| 🚀 **Data Recovery**   | Replay mechanism done on WAL     |  ❌ |  
| 📮 **Compaction for SSTables**   | Merge multiple sstables for Memory optimization     |  ❌ |  
| 📊 **Data Replication**   | Data is split inside sstables and also in WAL     |  ❌ |  
| 🌐 **Network Availability**   | Public endpoints exposed for end users     |  ❌ |  
