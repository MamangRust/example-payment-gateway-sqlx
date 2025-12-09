export interface User {
  id: number;
  firstname: string;
  lastname: string;
  email: string;
  created_at: string;
  updated_at: string;
}

export interface UserTrashed {
  id: number;
  firstname: string;
  lastname: string;
  email: string;
  created_at: string;
  updated_at: string;
  deleted_at: string;
}

