from ctypes import cdll

lib = cdll.LoadLibrary("target/release/libcallbyother.so")

lib.process()

print("Python Done!")
