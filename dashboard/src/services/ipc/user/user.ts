import {
  CreateUser,
  FindAllUser,
  FindAllUserTrashed,
  FindByIdUser,
  TrashedUser,
  UpdateUser,
} from "@/types/domain/request";
import {
  ApiResponsePaginationUser,
  ApiResponsePaginationUserDeleteAt,
  ApiResponseUser,
} from "@/types/domain/response";
import { invoke } from "@tauri-apps/api/core";

class UserCommand {
  async findAllUsers(
    accessToken: string,
    req: FindAllUser,
  ): Promise<ApiResponsePaginationUser> {
    try {
      return await invoke("find_all_users", {
        accessToken,
        req,
      });
    } catch (error) {
      console.error("Error in findAllUsers:", error);
      throw error;
    }
  }

  async findByIdUser(
    accessToken: string,
    req: FindByIdUser,
  ): Promise<ApiResponseUser> {
    try {
      return await invoke("find_by_id_user", {
        accessToken,
        req,
      });
    } catch (error) {
      console.error("Error in findByIdUser:", error);
      throw error;
    }
  }

  async findByActive(
    accessToken: string,
    req: FindAllUserTrashed,
  ): Promise<ApiResponsePaginationUserDeleteAt> {
    try {
      return await invoke("find_by_active", {
        accessToken,
        req,
      });
    } catch (error) {
      console.error("Error in findByActive:", error);
      throw error;
    }
  }

  async createUser(
    accessToken: string,
    req: CreateUser,
  ): Promise<ApiResponseUser> {
    try {
      return await invoke("create_user", {
        accessToken,
        req,
      });
    } catch (error) {
      console.error("Error in createUser:", error);
      throw error;
    }
  }

  async updateUser(
    accessToken: string,
    req: UpdateUser,
  ): Promise<ApiResponseUser> {
    try {
      return await invoke("update_user", {
        accessToken,
        req,
      });
    } catch (error) {
      console.error("Error in updateUser:", error);
      throw error;
    }
  }

  async trashedUser(
    accessToken: string,
    req: TrashedUser,
  ): Promise<ApiResponseUser> {
    try {
      return await invoke("trashed_user", {
        accessToken,
        req,
      });
    } catch (error) {
      console.error("Error in trashedUser:", error);
      throw error;
    }
  }
}

export default new UserCommand();
