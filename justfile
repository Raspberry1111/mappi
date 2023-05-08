dynamic_link_flags := "--features dynamic_linking"

alias r := run
alias rd := run-dyn
alias b := build
alias bd := build-dyn
alias t := test
alias td := test-dyn

run-dyn *flags: (run flags dynamic_link_flags)
build-dyn *flags: (build flags dynamic_link_flags)
test-dyn *flags: (test flags dynamic_link_flags) 

run *flags:
	cargo run {{flags}}

build *flags:
	cargo build {{flags}}

test *flags:
	cargo nextest {{flags}}