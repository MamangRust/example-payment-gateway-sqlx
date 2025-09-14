import useRoleStore from "@/store/role/role";
import { useNavigate } from "react-router-dom";
import { useToast } from "@/hooks/use-toast";
import { z } from "zod";
import useModalRole from "@/store/role/modal";
import { TrashedRole } from "@/types/domain/request";

export default function useDeleteRole() {
  const {
    deleteRoleId,
    isModalVisibleDelete,
    showModalDelete,
    hideModalDelete,
  } = useModalRole();

  const {
    trashedRole,
    setLoadingTrashedRole,
    loadingTrashedRole,
    setErrorTrashedRole,
  } = useRoleStore();

  const navigate = useNavigate();
  const { toast } = useToast();

  const handleSubmit = async () => {
    setLoadingTrashedRole(true);

    try {
      await new Promise((resolve) => setTimeout(resolve, 1200));

      const req: TrashedRole = {
        id: deleteRoleId as number,
        toast: toast,
      };

      const result = await trashedRole(req);

      if (result) {
        toast({
          title: "Success",
          description: "Role berhasil didelete",
          variant: "default",
        });

        navigate(window.location.pathname);
      } else {
        toast({
          title: "Error",
          description: "Gagal mendelete Role. Silakan coba lagi.",
          variant: "destructive",
        });
      }
    } catch (error: any) {
      if (error instanceof z.ZodError) {
        const errorMessage = error.errors.map((err) => err.message).join(", ");
        setErrorTrashedRole(errorMessage);
        toast({
          title: "Validation Error",
          description: errorMessage,
          variant: "destructive",
        });
      } else {
        setErrorTrashedRole(
          error?.message || "Terjadi kesalahan saat mengedit Role",
        );
        toast({
          title: "Error",
          description: error?.message || "Terjadi kesalahan saat mengedit Role",
          variant: "destructive",
        });
      }
    } finally {
      setLoadingTrashedRole(false);
      hideModalDelete();
    }
  };

  return {
    deleteRoleId,
    handleSubmit,
    loadingTrashedRole,
    isModalVisibleDelete,
    showModalDelete,
    hideModalDelete,
  };
}
