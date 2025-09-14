import { useNavigate } from "react-router-dom";
import { useToast } from "@/hooks/use-toast";
import useModalWithdrawTrashed from "@/store/withdraw/trashed/modal";
import useWithdrawTrashedStore from "@/store/withdraw/trashed/trashed";
import { DeletePermanentWithdraw } from "@/types/domain/request";

export default function useDeletePermanentWithdraw() {
  const {
    deletePermanentWithdrawId,
    isModalVisibleDeletePermanent,
    showModalDeletePermanent,
    hideModalDeletePermanent,
  } = useModalWithdrawTrashed();

  const {
    deletePermanentWithdraw,
    setLoadingDeletePermanentWithdrawTrashed,
    loadingDeletePermanentWithdrawTrashed,
    setErrorDeletePermanentWithdrawTrashed,
  } = useWithdrawTrashedStore();

  const navigate = useNavigate();
  const { toast } = useToast();

  const handleSubmit = async () => {
    setLoadingDeletePermanentWithdrawTrashed(true);

    try {
      await new Promise((resolve) => setTimeout(resolve, 1200));

      const req: DeletePermanentWithdraw = {
        id: deletePermanentWithdrawId as number,
        toast: toast,
      };

      const result = await deletePermanentWithdraw(req);

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
          description: "Gagal menghapus Withdraw. Silakan coba lagi.",
          variant: "destructive",
        });
      }
    } catch (error: any) {
      const errorMessage =
        error?.message || "Terjadi kesalahan saat menghapus Withdraw";
      setErrorDeletePermanentWithdrawTrashed(errorMessage);
      toast({
        title: "Error",
        description: errorMessage,
        variant: "destructive",
      });
    } finally {
      setLoadingDeletePermanentWithdrawTrashed(false);
      hideModalDeletePermanent();
    }
  };

  return {
    deletePermanentWithdrawId,
    handleSubmit,
    loadingDeletePermanentWithdrawTrashed,
    isModalVisibleDeletePermanent,
    showModalDeletePermanent,
    hideModalDeletePermanent,
  };
}
