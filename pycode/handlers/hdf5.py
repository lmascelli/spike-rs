from . import libnative_c as lb

class PhaseNC:
  def __init__(self, filename: str):
    lb.init()
    self.handler = lb.PyPhase(filename)

  def get(self):
    return self.handler

  def __del__(self):
    lb.close()
