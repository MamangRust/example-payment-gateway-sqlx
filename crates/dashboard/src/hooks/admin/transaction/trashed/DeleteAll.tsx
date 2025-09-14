import { useNavigate } from "react-router-dom";
import { useToast } from "@/hooks/use-toast";
import useModalTransactionTrashed from "@/store/transaction/trashed/modal";
import useTransactionTrashedStore from "@/store/transaction/trashed/trashed";

export default function useDeletePermanentAllTransaction() {
  const {
    isModalVisibleDeletePermanentAll,
    showModalDeletePermanentAll,
    hideModalDeletePermanentAll,
  } = useModalTransactionTrashed();

  const {
    deletePermanentAllTransaction,
    setLoadingDeletePermanentAllTransactionTrashed,
    loadingDeletePermanentAllTransactionTrashed,
    setErrorDeletePermanentAllTransactionTrashed,
  } = useTransactionTrashedStore();

  const navigate = useNavigate();
  const { toast } = useToast();

  const handleSubmit = async () => {
    setLoadingDeletePermanentAllTransactionTrashed(true);

    try {
      await new Promise((resolve) => setTimeout(resolve, 1200));

      const result = await deletePermanentAllTransaction(toast);

      if (result) {
        toast({
          title: "Success",
          description: "Transaction berhasil dihapus",
          variant: "default",
        });

        navigate(window.location.pathname);
      } else {
        toast({
          title: "Error",
          description: "Gagal menghapus Transaction. Silakan coba lagi.",
          variant: "destructive",
        });
      }
    } catch (error: any) {
      const errorMessage =
        error?.message || "Terjadi kesalahan saat menghapus Transaction";
      setErrorDeletePermanentAllTransactionTrashed(errorMessage);
      toast({
        title: "Error",
        description: errorMessage,
        variant: "destructive",
      });
    } finally {
      setLoadingDeletePermanentAllTransactionTrashed(false);
      hideModalDeletePermanentAll();
    }
  };

  return {
    handleSubmit,
    loadingDeletePermanentAllTransactionTrashed,
    isModalVisibleDeletePermanentAll,
    showModalDeletePermanentAll,
    hideModalDeletePermanentAll,
  };
}
