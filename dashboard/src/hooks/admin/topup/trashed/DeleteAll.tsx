import { useNavigate } from "react-router-dom";
import { useToast } from "@/hooks/use-toast";
import useModalTopupTrashed from "@/store/topup/trashed/modal";
import useTopupTrashedStore from "@/store/topup/trashed/trashed";

export default function useDeletePermanentAllTopup() {
  const {
    isModalVisibleDeletePermanentAll,
    showModalDeletePermanentAll,
    hideModalDeletePermanentAll,
  } = useModalTopupTrashed();

  const {
    deletePermanentAllTopup,
    setLoadingDeletePermanentAllTopupTrashed,
    loadingDeletePermanentAllTopupTrashed,
    setErrorDeletePermanentAllTopupTrashed,
  } = useTopupTrashedStore();

  const navigate = useNavigate();
  const { toast } = useToast();

  const handleSubmit = async () => {
    setLoadingDeletePermanentAllTopupTrashed(true);

    try {
      await new Promise((resolve) => setTimeout(resolve, 1200));

      const result = await deletePermanentAllTopup(toast);

      if (result) {
        toast({
          title: "Success",
          description: "Topup berhasil dihapus",
          variant: "default",
        });

        navigate(window.location.pathname);
      } else {
        toast({
          title: "Error",
          description: "Gagal menghapus Topup. Silakan coba lagi.",
          variant: "destructive",
        });
      }
    } catch (error: any) {
      const errorMessage =
        error?.message || "Terjadi kesalahan saat menghapus Topup";
      setErrorDeletePermanentAllTopupTrashed(errorMessage);
      toast({
        title: "Error",
        description: errorMessage,
        variant: "destructive",
      });
    } finally {
      setLoadingDeletePermanentAllTopupTrashed(false);
      hideModalDeletePermanentAll();
    }
  };

  return {
    handleSubmit,
    loadingDeletePermanentAllTopupTrashed,
    isModalVisibleDeletePermanentAll,
    showModalDeletePermanentAll,
    hideModalDeletePermanentAll,
  };
}
