

export interface Role {
  id: number;
  name: string;
  created_at: string;
  updated_at: string;
}

export interface RoleTrashed extends Role {
  deleted_at: string;
}

