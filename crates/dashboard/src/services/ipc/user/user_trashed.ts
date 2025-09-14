import {
  FindAllUser,
  RestoreUserTrashed,
  DeletePermanentUser,
} from "@/types/domain/request";
import {
  ApiResponseUser,
  ApiResponseUserAll,
  ApiResponseUserDelete,
  ApiResponsePaginationUserDeleteAt,
} from "@/types/domain/response";
import { invoke } from "@tauri-apps/api/core";

class UserTrashedCommand {
  async findAllUsersTrashed(
    accessToken: string,
    req: FindAllUser,
  ): Promise<ApiResponsePaginationUserDeleteAt> {
    try {
      return await invoke("find_all_users_trashed", { accessToken, req });
    } catch (error) {
      console.error("Error in findAllUsersTrashed:", error);
      throw error;
    }
  }

  async restoreUserTrashed(
    accessToken: string,
    req: RestoreUserTrashed,
  ): Promise<ApiResponseUser> {
    try {
      return await invoke("restore_user_trashed", { accessToken, req });
    } catch (error) {
      console.error("Error in restoreUserTrashed:", error);
      throw error;
    }
  }

  async deletePermanentUser(
    accessToken: string,
    req: DeletePermanentUser,
  ): Promise<ApiResponseUserDelete> {
    try {
      return await invoke("delete_permanent_user", { accessToken, req });
    } catch (error) {
      console.error("Error in deletePermanentUser:", error);
      throw error;
    }
  }

  async restoreUserAllTrashed(
    accessToken: string,
  ): Promise<ApiResponseUserAll> {
    try {
      return await invoke("restore_user_all_trashed", { accessToken });
    } catch (error) {
      console.error("Error in restoreUserAllTrashed:", error);
      throw error;
    }
  }

  async deletePermanentAllUser(
    accessToken: string,
  ): Promise<ApiResponseUserAll> {
    try {
      return await invoke("delete_permanent_all_user", { accessToken });
    } catch (error) {
      console.error("Error in deletePermanentAllUser:", error);
      throw error;
    }
  }
}

export default new UserTrashedCommand();
