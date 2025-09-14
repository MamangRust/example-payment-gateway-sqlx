import { User, UserTrashed } from "@/types/model";
import { PaginationMeta } from "../pagination";

export interface ApiResponseUser {
  status: string;
  message: string;
  data: User;
}

export interface ApiResponsesUser {
  status: string;
  message: string;
  data: User[];
}

export interface ApiResponseUserDelete {
  status: string;
  message: string;
}

export interface ApiResponseUserAll {
  status: string;
  message: string;
}

export interface ApiResponsePaginationUserDeleteAt {
  status: string;
  message: string;
  data: UserTrashed[];
  pagination: PaginationMeta;
}

export interface ApiResponsePaginationUser {
  status: string;
  message: string;
  data: User[];
  pagination: PaginationMeta;
}
