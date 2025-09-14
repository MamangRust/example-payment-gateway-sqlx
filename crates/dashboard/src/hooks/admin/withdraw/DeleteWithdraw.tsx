import useWithdrawStore from "@/store/withdraw/withdraw";
import { useNavigate } from "react-router-dom";
import { useToast } from "@/hooks/use-toast";
import { z } from "zod";
import useModalWithdraw from "@/store/withdraw/modal";
import { TrashedWithdraw } from "@/types/domain/request";

export default function useDeleteWithdraw() {
  const {
    deleteWithdrawId,
    isModalVisibleDelete,
    showModalDelete,
    hideModalDelete,
  } = useModalWithdraw();

  const {
    trashedWithdraw,
    setLoadingTrashedWithdraw,
    loadingTrashedWithdraw,
    setErrorTrashedWithdraw,
  } = useWithdrawStore();

  const navigate = useNavigate();
  const { toast } = useToast();

  const handleSubmit = async () => {
    setLoadingTrashedWithdraw(true);

    try {
      await new Promise((resolve) => setTimeout(resolve, 1200));

      const req: TrashedWithdraw = {
        id: deleteWithdrawId as number,
        toast: toast,
      };

      const result = await trashedWithdraw(req);

      if (result) {
        toast({
          title: "Success",
          description: "Withdraw berhasil dihapus",
          variant: "default",
        });

        navigate(window.location.pathname);
      } else {
        toast({
          title: "Error",
          description: "Gagal menghapus withdraw. Silakan coba lagi.",
          variant: "destructive",
        });
      }
    } catch (error: any) {
      if (error instanceof z.ZodError) {
        const errorMessage = error.errors.map((err) => err.message).join(", ");
        setErrorTrashedWithdraw(errorMessage);
        toast({
          title: "Validation Error",
          description: errorMessage,
          variant: "destructive",
        });
      } else {
        setErrorTrashedWithdraw(
          error?.message || "Terjadi kesalahan saat menghapus withdraw",
        );
        toast({
          title: "Error",
          description:
            error?.message || "Terjadi kesalahan saat menghapus withdraw",
          variant: "destructive",
        });
      }
    } finally {
      setLoadingTrashedWithdraw(false);
      hideModalDelete();
    }
  };

  return {
    deleteWithdrawId,
    handleSubmit,
    loadingTrashedWithdraw,
    isModalVisibleDelete,
    showModalDelete,
    hideModalDelete,
  };
}
