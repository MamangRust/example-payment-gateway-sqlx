import { LoginFormValues } from "@/schemas/auth/login";
import { SubmitHandler } from "react-hook-form";

export interface LoginFormProps {
  handleSubmit: SubmitHandler<LoginFormValues>;
  loadingLogin: boolean;
}
