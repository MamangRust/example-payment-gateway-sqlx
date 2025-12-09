import { useEffect, useRef } from "react";
import useRoleStore from "@/store/role/role";
import { useNavigate } from "react-router-dom";
import { useToast } from "@/hooks/use-toast";
import { UpdateRoleFormValues, updateRoleRequestSchema } from "@/schemas";
import { z } from "zod";
import useModalRole from "@/store/role/modal";
import { FindByIdRole, UpdateRole } from "@/types/domain/request";

export default function useUpdateRole() {
  const { isModalVisibleEdit, showModalEdit, hideModalEdit, editRoleId } =
    useModalRole();

  const {
    updateRole,
    setLoadingUpdateRole,
    loadingUpdateRole,
    setErrorUpdateRole,
    Role,
    findById,
  } = useRoleStore();

  const navigate = useNavigate();
  const { toast } = useToast();

  const formRef = useRef<HTMLFormElement>(null);

  useEffect(() => {
    if (isModalVisibleEdit && editRoleId !== null) {
      const req: FindByIdRole = {
        toast,
        id: editRoleId,
      };

      findById(req);
    }
  }, [isModalVisibleEdit, editRoleId, findById]);

  const handleButtonSubmit = () => {
    formRef.current?.requestSubmit();
  };

  const handleSubmit = async (data: UpdateRoleFormValues) => {
    setLoadingUpdateRole(true);

    try {
      const validatedValues = updateRoleRequestSchema.parse(data);

      await new Promise((resolve) => setTimeout(resolve, 1200));

      const req: UpdateRole = {
        role_id: editRoleId as number,
        name: validatedValues.name,
        toast: toast,
      };

      const result = await updateRole(req);

      if (result) {
        toast({
          title: "Success",
          description: "Role berhasil diupdate",
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
        setErrorUpdateRole(errorMessage);
        toast({
          title: "Validation Error",
          description: errorMessage,
          variant: "destructive",
        });
      } else {
        setErrorUpdateRole(
          error?.message || "Terjadi kesalahan saat mengedit Role",
        );
        toast({
          title: "Error",
          description: error?.message || "Terjadi kesalahan saat mengedit Role",
          variant: "destructive",
        });
      }
    } finally {
      setLoadingUpdateRole(false);
      hideModalEdit();
    }
  };

  return {
    editRoleId,
    formRef,
    Role,
    handleButtonSubmit,
    handleSubmit,
    loadingUpdateRole,
    isModalVisibleEdit,
    showModalEdit,
    hideModalEdit,
  };
}
