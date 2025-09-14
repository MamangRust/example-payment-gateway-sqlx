import useUserStore from "@/store/user/user";
import { useNavigate } from "react-router-dom";
import { useToast } from "@/hooks/use-toast";
import { z } from "zod";
import useModalUser from "@/store/user/modal";
import { TrashedUser } from "@/types/domain/request";

export default function useDeleteUser() {
  const {
    deleteUserId,
    isModalVisibleDelete,
    showModalDelete,
    hideModalDelete,
  } = useModalUser();

  const {
    trashedUser,
    setLoadingTrashedUser,
    loadingTrashedUser,
    setErrorTrashedUser,
  } = useUserStore();

  const navigate = useNavigate();
  const { toast } = useToast();

  const handleSubmit = async () => {
    setLoadingTrashedUser(true);

    try {
      await new Promise((resolve) => setTimeout(resolve, 1200));

      const req: TrashedUser = {
        id: deleteUserId as number,
        toast: toast,
      };

      const result = await trashedUser(req);

      if (result) {
        toast({
          title: "Success",
          description: "User berhasil didelete",
          variant: "default",
        });

        navigate(window.location.pathname);
      } else {
        toast({
          title: "Error",
          description: "Gagal mendelete user. Silakan coba lagi.",
          variant: "destructive",
        });
      }
    } catch (error: any) {
      if (error instanceof z.ZodError) {
        const errorMessage = error.errors.map((err) => err.message).join(", ");
        setErrorTrashedUser(errorMessage);
        toast({
          title: "Validation Error",
          description: errorMessage,
          variant: "destructive",
        });
      } else {
        setErrorTrashedUser(
          error?.message || "Terjadi kesalahan saat mengedit user",
        );
        toast({
          title: "Error",
          description: error?.message || "Terjadi kesalahan saat mengedit user",
          variant: "destructive",
        });
      }
    } finally {
      setLoadingTrashedUser(false);
      hideModalDelete();
    }
  };

  return {
    deleteUserId,
    handleSubmit,
    loadingTrashedUser,
    isModalVisibleDelete,
    showModalDelete,
    hideModalDelete,
  };
}
