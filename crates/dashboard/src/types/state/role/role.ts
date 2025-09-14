import {
  CreateRole,
  FindAllRole,
  FindByIdRole,
  TrashedRole,
  UpdateRole,
} from "@/types/domain/request";
import { Role } from "@/types/model/role";

export interface RoleStore {
  Roles: Role[] | null;
  Role: Role | null;

  pagination: {
    currentPage: number;
    page_size: number;
    totalItems: number;
    totalPages: number;
  };
  loadingGetRoles: boolean;
  loadingGetRole: boolean;
  loadingGetActiveRoles: boolean;
  loadingGetTrashedRoles: boolean;

  loadingCreateRole: boolean;
  loadingUpdateRole: boolean;
  loadingTrashedRole: boolean;

  errorGetRoles: string | null;
  errorGetRole: string | null;
  errorGetActiveRoles: string | null;
  errorGetTrashedRoles: string | null;
  errorCreateRole: string | null;
  errorUpdateRole: string | null;
  errorTrashedRole: string | null;

  setErrorGetRoles: (value: string | null) => void;
  setErrorGetRole: (value: string | null) => void;
  setErrorGetActiveRoles: (value: string | null) => void;
  setErrorGetTrashedRoles: (value: string | null) => void;

  setErrorCreateRole: (value: string | null) => void;
  setErrorUpdateRole: (value: string | null) => void;
  setErrorTrashedRole: (value: string | null) => void;

  setLoadingGetRoles: (value: boolean) => void;
  setLoadingGetRole: (value: boolean) => void;
  setLoadingGetActiveRoles: (value: boolean) => void;
  setLoadingGetTrashedRoles: (value: boolean) => void;

  setLoadingCreateRole: (value: boolean) => void;
  setLoadingUpdateRole: (value: boolean) => void;
  setLoadingTrashedRole: (value: boolean) => void;

  findAllRoles: (req: FindAllRole) => Promise<void>;
  findById: (req: FindByIdRole) => Promise<void>;
  findByActive: (req: FindAllRole) => Promise<void>;

  createRole: (req: CreateRole) => Promise<boolean>;
  updateRole: (req: UpdateRole) => Promise<boolean>;
  trashedRole: (req: TrashedRole) => Promise<boolean>;
}
