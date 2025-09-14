import { useNavigate } from "react-router-dom";
import { useToast } from "@/hooks/use-toast";
import useModalWithdrawTrashed from "@/store/withdraw/trashed/modal";
import useWithdrawTrashedStore from "@/store/withdraw/trashed/trashed";

export default function useDeletePermanentAllWithdraw() {
  const {
    isModalVisibleDeletePermanentAll,
    showModalDeletePermanentAll,
    hideModalDeletePermanentAll,
  } = useModalWithdrawTrashed();

  const {
    deletePermanentAllWithdraw,
    setLoadingDeletePermanentAllWithdrawTrashed,
    loadingDeletePermanentAllWithdrawTrashed,
    setErrorDeletePermanentAllWithdrawTrashed,
  } = useWithdrawTrashedStore();

  const navigate = useNavigate();
  const { toast } = useToast();

  const handleSubmit = async () => {
    setLoadingDeletePermanentAllWithdrawTrashed(true);

    try {
      await new Promise((resolve) => setTimeout(resolve, 1200));

      const result = await deletePermanentAllWithdraw(toast);

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
      setErrorDeletePermanentAllWithdrawTrashed(errorMessage);
      toast({
        title: "Error",
        description: errorMessage,
        variant: "destructive",
      });
    } finally {
      setLoadingDeletePermanentAllWithdrawTrashed(false);
      hideModalDeletePermanentAll();
    }
  };

  return {
    handleSubmit,
    loadingDeletePermanentAllWithdrawTrashed,
    isModalVisibleDeletePermanentAll,
    showModalDeletePermanentAll,
    hideModalDeletePermanentAll,
  };
}
