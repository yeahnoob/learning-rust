#!/usr/bin/env perl

use Modern::Perl;
use warnings;

use FFI::Raw;

my $rust_process = FFI::Raw->new(
    "target/release/libcallbyother.so", # library
    "process", # function name
    FFI::Raw::ptr, # return type
#    FFI::Raw::int, FFI::Raw::int # argument types
);


my $callrust = $rust_process->call();

print "Perl Done!\n";
# no more
