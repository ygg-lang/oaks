class Person:
    """A simple Person class."""
    
    def __init__(self, name, age):
        self.name = name
        self.age = age
    
    def greet(self):
        return f"Hello, my name is {self.name}"
    
    def get_age(self):
        return self.age
    
    @property
    def info(self):
        return {"name": self.name, "age": self.age}

class Student(Person):
    def __init__(self, name, age, student_id):
        super().__init__(name, age)
        self.student_id = student_id
    
    def study(self, subject):
        print(f"{self.name} is studying {subject}")