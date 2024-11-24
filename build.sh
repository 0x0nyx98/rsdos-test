export CFLAGS=-fno-plt
export CXXFLAGS=-fno-plt
cargo build

nasm -f obj asm/stack.s -o transient/stack.obj

export PATH=$PATH:$WATCOM/binl:$WATCOM/binw
wlink name outbox/rstest system dos32a option dosseg option start=main file transient/stack.obj library target/dos/debug/librsdos_test.a