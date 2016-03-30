#!python


env = Environment(tools=['mingw'], CFLAGS='-Wall --std=c99')

source=[
    'src/main.c',
    'src/scanner.c',
    'src/lexer.c',
    ]

env.Program(target='build/vuur.exe', source=source)
