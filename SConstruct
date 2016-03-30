#!python

import os

tools = []

if os.name == 'nt':
    tools.append('mingw')
else:
    tools.append('default')

env = Environment(
    tools=tools, 
    CFLAGS='-Wall --std=c99',
    ENV={ 'PATH' : os.environ.get('PATH') },
    )

source=[
    'src/main.c',
    'src/scanner.c',
    'src/lexer.c',
    ]

env.Program(target='build/vuur', source=source)
