import {
  FindAllRole,
  RestoreRoleTrashed,
  DeletePermanentRole,
} from "@/types/domain/request";
import {
  ApiResponseRole,
  ApiResponseRoleAll,
  ApiResponseRoleDelete,
  ApiResponsePaginationRoleDeleteAt,
} from "@/types/domain/response";
import { invoke } from "@tauri-apps/api/core";

class RoleTrashedCommand {
  async findAllRolesTrashed(
    accessToken: string,
    req: FindAllRole,
  ): Promise<ApiResponsePaginationRoleDeleteAt> {
    try {
      return await invoke("find_all_roles_trashed", { accessToken, req });
    } catch (error) {
      console.error("Error in findAllRolesTrashed:", error);
      throw error;
    }
  }

  async restoreRoleTrashed(
    accessToken: string,
    req: RestoreRoleTrashed,
  ): Promise<ApiResponseRole> {
    try {
      return await invoke("restore_role_trashed", { accessToken, req });
    } catch (error) {
      console.error("Error in restoreRoleTrashed:", error);
      throw error;
    }
  }

  async deletePermanentRole(
    accessToken: string,
    req: DeletePermanentRole,
  ): Promise<ApiResponseRoleDelete> {
    try {
      return await invoke("delete_permanent_role", { accessToken, req });
    } catch (error) {
      console.error("Error in deletePermanentRole:", error);
      throw error;
    }
  }

  async restoreRoleAllTrashed(
    accessToken: string,
  ): Promise<ApiResponseRoleAll> {
    try {
      return await invoke("restore_role_all_trashed", { accessToken });
    } catch (error) {
      console.error("Error in restoreRoleAllTrashed:", error);
      throw error;
    }
  }

  async deletePermanentAllRole(
    accessToken: string,
  ): Promise<ApiResponseRoleAll> {
    try {
      return await invoke("delete_permanent_all_role", { accessToken });
    } catch (error) {
      console.error("Error in deletePermanentAllRole:", error);
      throw error;
    }
  }
}

export default new RoleTrashedCommand();
