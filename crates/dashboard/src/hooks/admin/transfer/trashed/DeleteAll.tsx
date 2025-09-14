import { useNavigate } from "react-router-dom";
import { useToast } from "@/hooks/use-toast";
import useModalTransaferTrashed from "@/store/transfer/trashed/modal";
import useTransferTrashedStore from "@/store/transfer/trashed/trashed";

export default function useDeletePermanentAllTransfer() {
  const {
    isModalVisibleDeletePermanentAll,
    showModalDeletePermanentAll,
    hideModalDeletePermanentAll,
  } = useModalTransaferTrashed();

  const {
    deletePermanentAllTransfer,
    setLoadingDeletePermanentAllTransferTrashed,
    loadingDeletePermanentAllTransferTrashed,
    setErrorDeletePermanentAllTransferTrashed,
  } = useTransferTrashedStore();

  const navigate = useNavigate();
  const { toast } = useToast();

  const handleSubmit = async () => {
    setLoadingDeletePermanentAllTransferTrashed(true);

    try {
      await new Promise((resolve) => setTimeout(resolve, 1200));

      const result = await deletePermanentAllTransfer(toast);

      if (result) {
        toast({
          title: "Success",
          description: "Transfer berhasil dihapus",
          variant: "default",
        });

        navigate(window.location.pathname);
      } else {
        toast({
          title: "Error",
          description: "Gagal menghapus Transfer. Silakan coba lagi.",
          variant: "destructive",
        });
      }
    } catch (error: any) {
      const errorMessage =
        error?.message || "Terjadi kesalahan saat menghapus Transfer";
      setErrorDeletePermanentAllTransferTrashed(errorMessage);
      toast({
        title: "Error",
        description: errorMessage,
        variant: "destructive",
      });
    } finally {
      setLoadingDeletePermanentAllTransferTrashed(false);
      hideModalDeletePermanentAll();
    }
  };

  return {
    handleSubmit,
    loadingDeletePermanentAllTransferTrashed,
    isModalVisibleDeletePermanentAll,
    showModalDeletePermanentAll,
    hideModalDeletePermanentAll,
  };
}
