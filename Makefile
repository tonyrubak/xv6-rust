RUSTC = rustc

SRCDIR = ./src

OUTDIR = ./bin
RUSTFLAGS = --out-dir $(OUTDIR) -L ./bin
RLIBFLAGS = --crate-type=rlib
RSLIBFLAGS = -C lto -C debuginfo=2 --crate-type=staticlib

all: memlayout mmu prc spinlock types x86

types: $(SRCDIR)/types.rs
	$(RUSTC) $(RLIBFLAGS) $(RUSTFLAGS) --crate-name=types $(SRCDIR)/types.rs

memlayout: $(SRCDIR)/memlayout.rs types
	$(RUSTC) $(RLIBFLAGS) $(RUSTFLAGS) --crate-name=memlayout $(SRCDIR)/memlayout.rs

mmu: $(SRCDIR)/mmu.rs types
	$(RUSTC) $(RLIBFLAGS) $(RUSTFLAGS) --crate-name=mmu $(SRCDIR)/mmu.rs

prc: $(SRCDIR)/prc.rs types
	$(RUSTC) $(RLIBFLAGS) $(RUSTFLAGS) --crate-name=prc $(SRCDIR)/prc.rs

spinlock: $(SRCDIR)/spinlock.rs prc types memlayout mmu x86
	$(RUSTC) $(RUSTFLAGS) $(RSLIBFLAGS) $(SRCDIR)/spinlock.rs
	cp $(OUTDIR)/libspinlock.a ~/xv6-public/spinlock.o

x86: $(SRCDIR)/x86.rs types
	$(RUSTC) $(RLIBFLAGS) $(RUSTFLAGS) --crate-name=x86 $(SRCDIR)/x86.rs
