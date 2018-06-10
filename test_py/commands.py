from enum import Enum
import json
from json import JSONEncoder
import random

class CommandEncoder(JSONEncoder):
        def default(self, o):
            if type(o) is Move:
                return {"Move": o.__dict__}
            if type(o) is ChangeDir:
                return {"ChangeDirection": o.__dict__}
            return o.__dict__    

class Direction(Enum):
    UP="Up"
    DOWN="Down"
    LEFT="Left"
    RIGHT="Right"

class MessageContainer(object):
    def __init__(self, unit, cmd):
        self.unit = unit
        self.cmd = cmd
    
    @staticmethod
    def get_random(unit):
        cmds = [Move,]
        cmd = random.choice(cmds).get_random()
        return MessageContainer(unit,cmd)

class Move(object):
    def __init__(self, dir):
        self.direction = dir.value

    @staticmethod
    def get_random():
        dir = random.choice(list(Direction))
        return Move(dir)

class ChangeDir(object):
    def __init__(self, dir):
        self.newdir = dir.value

    @staticmethod
    def get_random():
        dir = random.choice(list(Direction))
        return ChangeDir(dir)

def main():
    cmd = Move(Direction.UP)
    mc = MessageContainer(2,cmd)
    print(CommandEncoder().encode(mc))
    cmd = ChangeDir(Direction.UP)
    mc = MessageContainer(2,cmd)
    print(CommandEncoder().encode(mc))
    mc = MessageContainer.get_random(2)
    print(CommandEncoder().encode(mc))


if __name__ == '__main__':
    main()