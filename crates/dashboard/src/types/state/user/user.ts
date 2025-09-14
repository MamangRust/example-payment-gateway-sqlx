import {
  CreateUser,
  FindAllUser,
  FindByIdUser,
  TrashedUser,
  UpdateUser,
} from "@/types/domain/request";
import { User } from "@/types/model/user";

export interface UserStore {
  users: User[] | null;
  user: User | null;

  pagination: {
    currentPage: number;
    page_size: number;
    totalItems: number;
    totalPages: number;
  };
  loadingGetUsers: boolean;
  loadingGetUser: boolean;
  loadingGetActiveUsers: boolean;
  loadingGetTrashedUsers: boolean;

  loadingCreateUser: boolean;
  loadingUpdateUser: boolean;
  loadingTrashedUser: boolean;

  errorGetUsers: string | null;
  errorGetUser: string | null;
  errorGetActiveUsers: string | null;
  errorGetTrashedUsers: string | null;
  errorCreateUser: string | null;
  errorUpdateUser: string | null;
  errorTrashedUser: string | null;

  setErrorGetUsers: (value: string | null) => void;
  setErrorGetUser: (value: string | null) => void;
  setErrorGetActiveUsers: (value: string | null) => void;
  setErrorGetTrashedUsers: (value: string | null) => void;

  setErrorCreateUser: (value: string | null) => void;
  setErrorUpdateUser: (value: string | null) => void;
  setErrorTrashedUser: (value: string | null) => void;

  setLoadingGetUsers: (value: boolean) => void;
  setLoadingGetUser: (value: boolean) => void;
  setLoadingGetActiveUsers: (value: boolean) => void;
  setLoadingGetTrashedUsers: (value: boolean) => void;

  setLoadingCreateUser: (value: boolean) => void;
  setLoadingUpdateUser: (value: boolean) => void;
  setLoadingTrashedUser: (value: boolean) => void;

  findAllUsers: (req: FindAllUser) => Promise<void>;
  findById: (req: FindByIdUser) => Promise<void>;
  findByActive: (req: FindAllUser) => Promise<void>;

  createUser: (req: CreateUser) => Promise<boolean>;
  updateUser: (req: UpdateUser) => Promise<boolean>;
  trashedUser: (req: TrashedUser) => Promise<boolean>;
}
