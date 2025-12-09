export interface UpdateUser {
  user_id: number;
  firstname: string;
  lastname: string;
  email: string;
  password: string;
  confirm_password: string;
  toast: any;
}
