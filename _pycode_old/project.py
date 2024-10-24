from pathlib import Path
from typing import Any, Dict, List
import json

# PHASE_TYPES
BASAL = 0
ULTRASOUND = 1
ELECTRICAL_STIM = 2


class Phase:
    def __init__(self, phase_path: Path):
        self.name = phase_path.name
        self.path = phase_path

    def write(self, json_data: Dict[str, Any]):
        if self.name in json_data.keys():
            print(f"The phase {self.name} already exists")
        else:
            json_data[self.name] = {"name": self.name, "path": str(self.path)}


class Batch:
    def __init__(self, name):
        self.name = name
        self.path = None
        self.phases: List[Phase] = []

    def write(self, json_data: Dict[str, Any]):
        if self.name in json_data.keys():
            print(f"The batch {self.name} already exists")
        else:
            phases_dict = {}
            for phase in self.phases:
                phase.write(phases_dict)
            json_data[self.name] = {
                "name": self.name,
                "path": str(self.path),
                "phases": phases_dict,
            }

    def add_phase(self, phase_path: Path):
        self.phases.append(Phase(phase_path))


class Project:
    def __init__(self):
        self.name = "New Project"
        self.batches: List[Batch] = []

    def write(self):
        json_data = {}
        json_data["name"] = self.name
        for batch in self.batches:
            batch.write(json_data)
        print(json.dumps(json_data, indent=""))

    def add_batch(self, batch_name: str):
        batch = Batch(batch_name)
        self.batches.append(batch)
