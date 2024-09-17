// automatically generated by the FlatBuffers compiler, do not modify


// @generated

use core::mem;
use core::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

pub enum ManifestOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Manifest<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Manifest<'a> {
  type Inner = Manifest<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> Manifest<'a> {
  pub const VT_FRAGMENTS: flatbuffers::VOffsetT = 4;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    Manifest { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args ManifestArgs<'args>
  ) -> flatbuffers::WIPOffset<Manifest<'bldr>> {
    let mut builder = ManifestBuilder::new(_fbb);
    if let Some(x) = args.fragments { builder.add_fragments(x); }
    builder.finish()
  }


  #[inline]
  pub fn fragments(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Fragment<'a>>>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Fragment>>>>(Manifest::VT_FRAGMENTS, None)}
  }
}

impl flatbuffers::Verifiable for Manifest<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<Fragment>>>>("fragments", Self::VT_FRAGMENTS, false)?
     .finish();
    Ok(())
  }
}
pub struct ManifestArgs<'a> {
    pub fragments: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Fragment<'a>>>>>,
}
impl<'a> Default for ManifestArgs<'a> {
  #[inline]
  fn default() -> Self {
    ManifestArgs {
      fragments: None,
    }
  }
}

pub struct ManifestBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> ManifestBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_fragments(&mut self, fragments: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<Fragment<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Manifest::VT_FRAGMENTS, fragments);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> ManifestBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    ManifestBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Manifest<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for Manifest<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("Manifest");
      ds.field("fragments", &self.fragments());
      ds.finish()
  }
}
pub enum FragmentOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Fragment<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Fragment<'a> {
  type Inner = Fragment<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> Fragment<'a> {
  pub const VT_NAME: flatbuffers::VOffsetT = 4;
  pub const VT_FILES: flatbuffers::VOffsetT = 6;
  pub const VT_BUNDLES: flatbuffers::VOffsetT = 8;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    Fragment { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args FragmentArgs<'args>
  ) -> flatbuffers::WIPOffset<Fragment<'bldr>> {
    let mut builder = FragmentBuilder::new(_fbb);
    if let Some(x) = args.bundles { builder.add_bundles(x); }
    if let Some(x) = args.files { builder.add_files(x); }
    if let Some(x) = args.name { builder.add_name(x); }
    builder.finish()
  }


  #[inline]
  pub fn name(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(Fragment::VT_NAME, None)}
  }
  #[inline]
  pub fn files(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<File<'a>>>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<File>>>>(Fragment::VT_FILES, None)}
  }
  #[inline]
  pub fn bundles(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Bundle<'a>>>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Bundle>>>>(Fragment::VT_BUNDLES, None)}
  }
}

impl flatbuffers::Verifiable for Fragment<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("name", Self::VT_NAME, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<File>>>>("files", Self::VT_FILES, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<Bundle>>>>("bundles", Self::VT_BUNDLES, false)?
     .finish();
    Ok(())
  }
}
pub struct FragmentArgs<'a> {
    pub name: Option<flatbuffers::WIPOffset<&'a str>>,
    pub files: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<File<'a>>>>>,
    pub bundles: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Bundle<'a>>>>>,
}
impl<'a> Default for FragmentArgs<'a> {
  #[inline]
  fn default() -> Self {
    FragmentArgs {
      name: None,
      files: None,
      bundles: None,
    }
  }
}

pub struct FragmentBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> FragmentBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_name(&mut self, name: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Fragment::VT_NAME, name);
  }
  #[inline]
  pub fn add_files(&mut self, files: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<File<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Fragment::VT_FILES, files);
  }
  #[inline]
  pub fn add_bundles(&mut self, bundles: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<Bundle<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Fragment::VT_BUNDLES, bundles);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> FragmentBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    FragmentBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Fragment<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for Fragment<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("Fragment");
      ds.field("name", &self.name());
      ds.field("files", &self.files());
      ds.field("bundles", &self.bundles());
      ds.finish()
  }
}
pub enum BundleOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Bundle<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Bundle<'a> {
  type Inner = Bundle<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> Bundle<'a> {
  pub const VT_HASH: flatbuffers::VOffsetT = 4;
  pub const VT_CHUNKS: flatbuffers::VOffsetT = 6;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    Bundle { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args BundleArgs<'args>
  ) -> flatbuffers::WIPOffset<Bundle<'bldr>> {
    let mut builder = BundleBuilder::new(_fbb);
    if let Some(x) = args.chunks { builder.add_chunks(x); }
    if let Some(x) = args.hash { builder.add_hash(x); }
    builder.finish()
  }


  #[inline]
  pub fn hash(&self) -> Option<flatbuffers::Vector<'a, i8>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, i8>>>(Bundle::VT_HASH, None)}
  }
  #[inline]
  pub fn chunks(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Chunk<'a>>>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Chunk>>>>(Bundle::VT_CHUNKS, None)}
  }
}

impl flatbuffers::Verifiable for Bundle<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, i8>>>("hash", Self::VT_HASH, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<Chunk>>>>("chunks", Self::VT_CHUNKS, false)?
     .finish();
    Ok(())
  }
}
pub struct BundleArgs<'a> {
    pub hash: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, i8>>>,
    pub chunks: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Chunk<'a>>>>>,
}
impl<'a> Default for BundleArgs<'a> {
  #[inline]
  fn default() -> Self {
    BundleArgs {
      hash: None,
      chunks: None,
    }
  }
}

pub struct BundleBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> BundleBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_hash(&mut self, hash: flatbuffers::WIPOffset<flatbuffers::Vector<'b , i8>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Bundle::VT_HASH, hash);
  }
  #[inline]
  pub fn add_chunks(&mut self, chunks: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<Chunk<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Bundle::VT_CHUNKS, chunks);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> BundleBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    BundleBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Bundle<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for Bundle<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("Bundle");
      ds.field("hash", &self.hash());
      ds.field("chunks", &self.chunks());
      ds.finish()
  }
}
pub enum FileOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct File<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for File<'a> {
  type Inner = File<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> File<'a> {
  pub const VT_NAME: flatbuffers::VOffsetT = 4;
  pub const VT_SIZE_: flatbuffers::VOffsetT = 6;
  pub const VT_HASH: flatbuffers::VOffsetT = 8;
  pub const VT_CHUNKS: flatbuffers::VOffsetT = 10;
  pub const VT_EXECUTABLE: flatbuffers::VOffsetT = 12;
  pub const VT_SYMLINK: flatbuffers::VOffsetT = 14;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    File { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args FileArgs<'args>
  ) -> flatbuffers::WIPOffset<File<'bldr>> {
    let mut builder = FileBuilder::new(_fbb);
    builder.add_size_(args.size_);
    if let Some(x) = args.symlink { builder.add_symlink(x); }
    if let Some(x) = args.chunks { builder.add_chunks(x); }
    if let Some(x) = args.hash { builder.add_hash(x); }
    if let Some(x) = args.name { builder.add_name(x); }
    builder.add_executable(args.executable);
    builder.finish()
  }


  #[inline]
  pub fn name(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(File::VT_NAME, None)}
  }
  #[inline]
  pub fn size_(&self) -> i64 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<i64>(File::VT_SIZE_, Some(0)).unwrap()}
  }
  #[inline]
  pub fn hash(&self) -> Option<flatbuffers::Vector<'a, i8>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, i8>>>(File::VT_HASH, None)}
  }
  #[inline]
  pub fn chunks(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Chunk<'a>>>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Chunk>>>>(File::VT_CHUNKS, None)}
  }
  #[inline]
  pub fn executable(&self) -> bool {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<bool>(File::VT_EXECUTABLE, Some(false)).unwrap()}
  }
  #[inline]
  pub fn symlink(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(File::VT_SYMLINK, None)}
  }
}

impl flatbuffers::Verifiable for File<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("name", Self::VT_NAME, false)?
     .visit_field::<i64>("size_", Self::VT_SIZE_, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, i8>>>("hash", Self::VT_HASH, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<Chunk>>>>("chunks", Self::VT_CHUNKS, false)?
     .visit_field::<bool>("executable", Self::VT_EXECUTABLE, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("symlink", Self::VT_SYMLINK, false)?
     .finish();
    Ok(())
  }
}
pub struct FileArgs<'a> {
    pub name: Option<flatbuffers::WIPOffset<&'a str>>,
    pub size_: i64,
    pub hash: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, i8>>>,
    pub chunks: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Chunk<'a>>>>>,
    pub executable: bool,
    pub symlink: Option<flatbuffers::WIPOffset<&'a str>>,
}
impl<'a> Default for FileArgs<'a> {
  #[inline]
  fn default() -> Self {
    FileArgs {
      name: None,
      size_: 0,
      hash: None,
      chunks: None,
      executable: false,
      symlink: None,
    }
  }
}

pub struct FileBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> FileBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_name(&mut self, name: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(File::VT_NAME, name);
  }
  #[inline]
  pub fn add_size_(&mut self, size_: i64) {
    self.fbb_.push_slot::<i64>(File::VT_SIZE_, size_, 0);
  }
  #[inline]
  pub fn add_hash(&mut self, hash: flatbuffers::WIPOffset<flatbuffers::Vector<'b , i8>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(File::VT_HASH, hash);
  }
  #[inline]
  pub fn add_chunks(&mut self, chunks: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<Chunk<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(File::VT_CHUNKS, chunks);
  }
  #[inline]
  pub fn add_executable(&mut self, executable: bool) {
    self.fbb_.push_slot::<bool>(File::VT_EXECUTABLE, executable, false);
  }
  #[inline]
  pub fn add_symlink(&mut self, symlink: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(File::VT_SYMLINK, symlink);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> FileBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    FileBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<File<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for File<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("File");
      ds.field("name", &self.name());
      ds.field("size_", &self.size_());
      ds.field("hash", &self.hash());
      ds.field("chunks", &self.chunks());
      ds.field("executable", &self.executable());
      ds.field("symlink", &self.symlink());
      ds.finish()
  }
}
pub enum ChunkOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Chunk<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Chunk<'a> {
  type Inner = Chunk<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> Chunk<'a> {
  pub const VT_HASH: flatbuffers::VOffsetT = 4;
  pub const VT_SIZE_: flatbuffers::VOffsetT = 6;
  pub const VT_OFFSET: flatbuffers::VOffsetT = 8;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    Chunk { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args ChunkArgs<'args>
  ) -> flatbuffers::WIPOffset<Chunk<'bldr>> {
    let mut builder = ChunkBuilder::new(_fbb);
    builder.add_offset(args.offset);
    builder.add_size_(args.size_);
    if let Some(x) = args.hash { builder.add_hash(x); }
    builder.finish()
  }


  #[inline]
  pub fn hash(&self) -> Option<flatbuffers::Vector<'a, i8>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, i8>>>(Chunk::VT_HASH, None)}
  }
  #[inline]
  pub fn size_(&self) -> i64 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<i64>(Chunk::VT_SIZE_, Some(0)).unwrap()}
  }
  #[inline]
  pub fn offset(&self) -> i64 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<i64>(Chunk::VT_OFFSET, Some(0)).unwrap()}
  }
}

impl flatbuffers::Verifiable for Chunk<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, i8>>>("hash", Self::VT_HASH, false)?
     .visit_field::<i64>("size_", Self::VT_SIZE_, false)?
     .visit_field::<i64>("offset", Self::VT_OFFSET, false)?
     .finish();
    Ok(())
  }
}
pub struct ChunkArgs<'a> {
    pub hash: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, i8>>>,
    pub size_: i64,
    pub offset: i64,
}
impl<'a> Default for ChunkArgs<'a> {
  #[inline]
  fn default() -> Self {
    ChunkArgs {
      hash: None,
      size_: 0,
      offset: 0,
    }
  }
}

pub struct ChunkBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> ChunkBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_hash(&mut self, hash: flatbuffers::WIPOffset<flatbuffers::Vector<'b , i8>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Chunk::VT_HASH, hash);
  }
  #[inline]
  pub fn add_size_(&mut self, size_: i64) {
    self.fbb_.push_slot::<i64>(Chunk::VT_SIZE_, size_, 0);
  }
  #[inline]
  pub fn add_offset(&mut self, offset: i64) {
    self.fbb_.push_slot::<i64>(Chunk::VT_OFFSET, offset, 0);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> ChunkBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    ChunkBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Chunk<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for Chunk<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("Chunk");
      ds.field("hash", &self.hash());
      ds.field("size_", &self.size_());
      ds.field("offset", &self.offset());
      ds.finish()
  }
}