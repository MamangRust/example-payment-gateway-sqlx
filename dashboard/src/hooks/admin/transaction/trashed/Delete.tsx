import { useNavigate } from "react-router-dom";
import { useToast } from "@/hooks/use-toast";
import useModalTransactionTrashed from "@/store/transaction/trashed/modal";
import useTransactionTrashedStore from "@/store/transaction/trashed/trashed";
import { DeletePermanentTransaction } from "@/types/domain/request";

export default function useDeletePermanentTransaction() {
  const {
    deletePermanentTransactionId,
    isModalVisibleDeletePermanent,
    showModalDeletePermanent,
    hideModalDeletePermanent,
  } = useModalTransactionTrashed();

  const {
    deletePermanentTransaction,
    setLoadingDeletePermanentTransactionTrashed,
    loadingDeletePermanentTransactionTrashed,
    setErrorDeletePermanentTransactionTrashed,
  } = useTransactionTrashedStore();

  const navigate = useNavigate();
  const { toast } = useToast();

  const handleSubmit = async () => {
    setLoadingDeletePermanentTransactionTrashed(true);

    try {
      await new Promise((resolve) => setTimeout(resolve, 1200));

      const req: DeletePermanentTransaction = {
        id: deletePermanentTransactionId as number,
        toast: toast,
      };

      const result = await deletePermanentTransaction(req);

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
      setErrorDeletePermanentTransactionTrashed(errorMessage);
      toast({
        title: "Error",
        description: errorMessage,
        variant: "destructive",
      });
    } finally {
      setLoadingDeletePermanentTransactionTrashed(false);
      hideModalDeletePermanent();
    }
  };

  return {
    deletePermanentTransactionId,
    handleSubmit,
    loadingDeletePermanentTransactionTrashed,
    isModalVisibleDeletePermanent,
    showModalDeletePermanent,
    hideModalDeletePermanent,
  };
}
