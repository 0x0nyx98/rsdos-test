cargo build
nasm -f obj asm/stack.s -o transient/stack.obj
export PATH=$PATH:$WATCOM/binl:$WATCOM/binw
wlink name outbox/rstest system dos32a option dosseg option start=_asm_start file transient/stack.obj