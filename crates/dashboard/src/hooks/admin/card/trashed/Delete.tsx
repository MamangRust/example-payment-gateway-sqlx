import { useNavigate } from "react-router-dom";
import { useToast } from "@/hooks/use-toast";
import useModalCardTrashed from "@/store/card/trashed/modal";
import useCardTrashedStore from "@/store/card/trashed/trashed";
import { DeletePermanentCard } from "@/types/domain/request";

export default function useDeletePermanentCard() {
  const {
    deletePermanentCardId,
    isModalVisibleDeletePermanent,
    showModalDeletePermanent,
    hideModalDeletePermanent,
  } = useModalCardTrashed();

  const {
    deletePermanentCard,
    setLoadingDeletePermanentCardTrashed,
    loadingDeletePermanentCardTrashed,
    setErrorDeletePermanentCardTrashed,
  } = useCardTrashedStore();

  const navigate = useNavigate();
  const { toast } = useToast();

  const handleSubmit = async () => {
    setLoadingDeletePermanentCardTrashed(true);

    try {
      await new Promise((resolve) => setTimeout(resolve, 1200));

      const req: DeletePermanentCard = {
        id: deletePermanentCardId as number,
        toast: toast,
      };

      const result = await deletePermanentCard(req);

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
      setErrorDeletePermanentCardTrashed(errorMessage);
      toast({
        title: "Error",
        description: errorMessage,
        variant: "destructive",
      });
    } finally {
      setLoadingDeletePermanentCardTrashed(false);
      hideModalDeletePermanent();
    }
  };

  return {
    handleSubmit,
    deletePermanentCardId,
    loadingDeletePermanentCardTrashed,
    isModalVisibleDeletePermanent,
    showModalDeletePermanent,
    hideModalDeletePermanent,
  };
}
