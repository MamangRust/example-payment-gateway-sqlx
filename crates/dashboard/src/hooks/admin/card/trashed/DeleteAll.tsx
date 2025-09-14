import { useNavigate } from "react-router-dom";
import { useToast } from "@/hooks/use-toast";
import useModalCardTrashed from "@/store/card/trashed/modal";
import useCardTrashedStore from "@/store/card/trashed/trashed";

export default function useDeletePermanentAllCard() {
  const {
    isModalVisibleDeletePermanentAll,
    showModalDeletePermanentAll,
    hideModalDeletePermanentAll,
  } = useModalCardTrashed();

  const {
    deletePermanentAllCard,
    setLoadingDeletePermanentAllCardTrashed,
    loadingDeletePermanentAllCardTrashed,
    setErrorDeletePermanentAllCardTrashed,
  } = useCardTrashedStore();

  const navigate = useNavigate();
  const { toast } = useToast();

  const handleSubmit = async () => {
    setLoadingDeletePermanentAllCardTrashed(true);

    try {
      await new Promise((resolve) => setTimeout(resolve, 1200));

      const result = await deletePermanentAllCard(toast);

      if (result) {
        toast({
          title: "Success",
          description: "Card berhasil dihapus",
          variant: "default",
        });

        navigate(window.location.pathname);
      } else {
        toast({
          title: "Error",
          description: "Gagal menghapus card. Silakan coba lagi.",
          variant: "destructive",
        });
      }
    } catch (error: any) {
      const errorMessage =
        error?.message || "Terjadi kesalahan saat menghapus card";
      setErrorDeletePermanentAllCardTrashed(errorMessage);
      toast({
        title: "Error",
        description: errorMessage,
        variant: "destructive",
      });
    } finally {
      setLoadingDeletePermanentAllCardTrashed(false);
      hideModalDeletePermanentAll();
    }
  };

  return {
    handleSubmit,
    loadingDeletePermanentAllCardTrashed,
    isModalVisibleDeletePermanentAll,
    showModalDeletePermanentAll,
    hideModalDeletePermanentAll,
  };
}
