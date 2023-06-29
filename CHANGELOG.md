# Changelog
## 0.1.1
- Added Into<TLV> for every TLV parser.
- Fixed Hostname parsing.
- Added features to toggle groups of TLV parsers.
## 0.1.2
- Reexported deku for trait inclusion.
## 0.1.3
- Added channel sequence.
## 0.2.1
- Migrated from deku to handwritten parsers
- Improved internal architecture and robustness
- Replaced any uses of std with alloc
- Introduced better Debug formatters for AWDLVersion and AWDLActionFrame
- Introduced PartialOrd for AWDLVersion
## 0.2.2
- Various performance improvements
- Introduced Service Response and Sync Tree TLVs
## 0.2.3
- Updated Bumpalo from 3.12.1 to 3.13.0
## 0.3.0
- Added iterator based serialization API.
- Introduced ServiceResponseTLV.
## 0.3.1
- Introduced HTCapabilitiesTLV.
- Inlined functions.
