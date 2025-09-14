import {
  CreateRole,
  FindAllRole,
  FindAllRoleTrashed,
  FindByIdRole,
  TrashedRole,
  UpdateRole,
} from "@/types/domain/request";
import {
  ApiResponsePaginationRole,
  ApiResponsePaginationRoleDeleteAt,
  ApiResponseRole,
} from "@/types/domain/response";
import { invoke } from "@tauri-apps/api/core";

class RoleCommand {
  async findAllRoles(
    accessToken: string,
    req: FindAllRole,
  ): Promise<ApiResponsePaginationRole> {
    try {
      return await invoke("find_all_roles", {
        accessToken,
        req,
      });
    } catch (error) {
      console.error("Error in findAllRoles:", error);
      throw error;
    }
  }

  async findByIdRole(
    accessToken: string,
    req: FindByIdRole,
  ): Promise<ApiResponseRole> {
    try {
      return await invoke("find_by_id_role", {
        accessToken,
        req,
      });
    } catch (error) {
      console.error("Error in findByIdRole:", error);
      throw error;
    }
  }

  async findByActive(
    accessToken: string,
    req: FindAllRoleTrashed,
  ): Promise<ApiResponsePaginationRoleDeleteAt> {
    try {
      return await invoke("find_by_active_role", {
        accessToken,
        req,
      });
    } catch (error) {
      console.error("Error in findByActive:", error);
      throw error;
    }
  }

  async createRole(
    accessToken: string,
    req: CreateRole,
  ): Promise<ApiResponseRole> {
    try {
      return await invoke("create_role", {
        accessToken,
        req,
      });
    } catch (error) {
      console.error("Error in createRole:", error);
      throw error;
    }
  }

  async updateRole(
    accessToken: string,
    req: UpdateRole,
  ): Promise<ApiResponseRole> {
    try {
      return await invoke("update_role", {
        accessToken,
        req,
      });
    } catch (error) {
      console.error("Error in updateRole:", error);
      throw error;
    }
  }

  async trashedRole(
    accessToken: string,
    req: TrashedRole,
  ): Promise<ApiResponseRole> {
    try {
      return await invoke("trashed_role", {
        accessToken,
        req,
      });
    } catch (error) {
      console.error("Error in trashedRole:", error);
      throw error;
    }
  }
}

export default new RoleCommand();
