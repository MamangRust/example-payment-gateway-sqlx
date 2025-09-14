import { useRef } from "react";
import useRoleStore from "@/store/role/role";
import { useNavigate } from "react-router-dom";
import { useToast } from "@/hooks/use-toast";
import { CreateRoleFormValues, createRoleRequestSchema } from "@/schemas";
import { z } from "zod";
import useModalRole from "@/store/role/modal";
import { CreateRole } from "@/types/domain/request";

export default function useCreateRole() {
  const { isModalVisible, showModal, hideModal } = useModalRole();

  const {
    createRole,
    setLoadingCreateRole,
    loadingCreateRole,
    setErrorCreateRole,
  } = useRoleStore();

  const navigate = useNavigate();
  const { toast } = useToast();

  const formRef = useRef<HTMLFormElement>(null);

  const handleButtonSubmit = () => {
    formRef.current?.requestSubmit();
  };

  const handleSubmit = async (data: CreateRoleFormValues) => {
    setLoadingCreateRole(true);

    try {
      const validatedValues = createRoleRequestSchema.parse(data);

      await new Promise((resolve) => setTimeout(resolve, 1200));

      const req: CreateRole = {
        name: validatedValues.name,
        toast: toast,
      };

      const result = await createRole(req);

      if (result) {
        toast({
          title: "Success",
          description: "Role berhasil dibuat",
          variant: "default",
        });

        navigate(window.location.pathname);
      } else {
        toast({
          title: "Error",
          description: "Gagal membuat Role. Silakan coba lagi.",
          variant: "destructive",
        });
      }
    } catch (error: any) {
      if (error instanceof z.ZodError) {
        const errorMessage = error.errors.map((err) => err.message).join(", ");
        setErrorCreateRole(errorMessage);
        toast({
          title: "Validation Error",
          description: errorMessage,
          variant: "destructive",
        });
      } else {
        setErrorCreateRole(
          error?.message || "Terjadi kesalahan saat membuat Role",
        );
        toast({
          title: "Error",
          description: error?.message || "Terjadi kesalahan saat membuat Role",
          variant: "destructive",
        });
      }
    } finally {
      setLoadingCreateRole(false);
      hideModal();
    }
  };

  return {
    formRef,
    handleButtonSubmit,
    handleSubmit,
    loadingCreateRole,
    isModalVisible,
    showModal,
    hideModal,
  };
}
