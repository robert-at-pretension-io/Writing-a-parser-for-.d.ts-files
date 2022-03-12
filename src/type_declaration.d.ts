interface Business {
  name: String;
  number_of_employees: number;
}

interface Organization {
  roles : String[];
}

type Employee = {
  name: String;
  role: String;
  salary: number;
};