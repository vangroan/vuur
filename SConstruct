#!python


env = Environment(tools=['mingw'], CFLAGS='-Wall')

source=[
    'src/main.c',
    'src/scanner.c',
    'src/token.c',
    'src/lexer.c',
    ]

env.Program(target='build/vuur.exe', source=source)

