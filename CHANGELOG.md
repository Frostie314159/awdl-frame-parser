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
