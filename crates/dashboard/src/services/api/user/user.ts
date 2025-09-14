import myApi from "@/helpers/api";
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

class UserService {
  async findAllUsers(
    access_token: string,
    req: FindAllUser,
  ): Promise<ApiResponsePaginationUser> {
    try {
      const response = await myApi.get(`/users`, {
        params: {
          page: req.page,
          page_size: req.page_size,
          search: req.search,
        },
        headers: {
          Authorization: `Bearer ${access_token}`,
        },
      });

      if (response.status == 200) {
        return response.data;
      }
      throw new Error(response.data.message || "Login failed.");
    } catch (error: any) {
      throw new Error(error.response?.data?.message || "Login failed.");
    }
  }
  async findById(
    access_token: string,
    req: FindByIdUser,
  ): Promise<ApiResponseUser> {
    try {
      const response = await myApi.get(`/users/${req.id}`, {
        headers: {
          Authorization: `Bearer ${access_token}`,
        },
      });

      if (response.status == 200) {
        return response.data.data;
      }
      throw new Error(response.data.message || "Login failed.");
    } catch (error: any) {
      throw new Error(error.response?.data?.message || "Login failed.");
    }
  }

  async findByActive(
    access_token: string,
    req: FindAllUserTrashed,
  ): Promise<ApiResponsePaginationUserDeleteAt> {
    try {
      const response = await myApi.get(`/users/active`, {
        params: {
          page: req.page,
          page_size: req.page_size,
          search: req.search,
        },
        headers: {
          Authorization: `Bearer ${access_token}`,
        },
      });

      if (response.status == 200) {
        return response.data;
      }
      throw new Error(response.data.message || "Login failed.");
    } catch (error: any) {
      throw new Error(error.response?.data?.message || "Login failed.");
    }
  }

  async createUser(
    access_token: string,
    req: CreateUser,
  ): Promise<ApiResponseUser> {
    try {
      const response = await myApi.post(
        `/users/create`,
        {
          firstname: req.firstname,
          lastname: req.lastname,
          email: req.email,
          password: req.password,
          confirm_password: req.confirm_password,
        },
        {
          headers: {
            Authorization: `Bearer ${access_token}`,
          },
        },
      );

      if (response.status == 200) {
        return response.data.data;
      }
      throw new Error(response.data.message || "Login failed.");
    } catch (error: any) {
      throw new Error(error.response?.data?.message || "Login failed.");
    }
  }
  async updateUser(
    access_token: string,
    req: UpdateUser,
  ): Promise<ApiResponseUser> {
    try {
      const response = await myApi.post(
        `/users/update/${req.user_id}`,
        {
          user_id: req.user_id,
          firstname: req.firstname,
          lastname: req.lastname,
          email: req.email,
          password: req.password,
          confirm_password: req.confirm_password,
        },
        {
          headers: {
            Authorization: `Bearer ${access_token}`,
          },
        },
      );

      if (response.status == 200) {
        return response.data.data;
      }
      throw new Error(response.data.message || "Login failed.");
    } catch (error: any) {
      throw new Error(error.response?.data?.message || "Login failed.");
    }
  }
  async trashedUser(
    access_token: string,
    req: TrashedUser,
  ): Promise<ApiResponseUser> {
    try {
      const response = await myApi.post(`/users/trashed/${req.id}`, null, {
        headers: {
          Authorization: `Bearer ${access_token}`,
        },
      });

      if (response.status == 200) {
        return response.data.data;
      }
      throw new Error(response.data.message || "Login failed.");
    } catch (error: any) {
      throw new Error(error.response?.data?.message || "Login failed.");
    }
  }
}

export default new UserService();
