import { calculatePercentageChange } from "@/helpers/calculate";
import { debounce } from "@/helpers/debounce";
import useTopupStore from "@/store/topup/topup";
import { useCallback } from "react";

export default function useYearStatusSuccessTopupDetail(currentYear: number, toast: any) {
    const {
        findYearStatusSuccessByCardNumber,
        yearStatusSuccess,
        loadingYearStatusSuccess,
        setLoadingYearStatusSuccess,
        setErrorYearStatusSuccess,
    } = useTopupStore();

    const currentYearSuccessData = yearStatusSuccess?.find(
        (balance) => balance.year === currentYear.toString(),
    );

    const previousYearSuccessData = yearStatusSuccess?.find(
        (balance) => balance.year === (currentYear - 1).toString(),
    );

    const currentYearSuccess = currentYearSuccessData?.total_amount || 0;
    const previousYearSucces = previousYearSuccessData?.total_amount || 0;

    const yearPercentageSuccessChange = calculatePercentageChange(
        currentYearSuccess,
        previousYearSucces,
    );

    const fetchYearlySuccess = useCallback(
        async (year: number, card_number: string) => {
            try {
                setLoadingYearStatusSuccess(true);
                setErrorYearStatusSuccess(null);

                await findYearStatusSuccessByCardNumber(toast, year, card_number);
            } catch (error) {
                setErrorYearStatusSuccess("Failed to fetch yearly balance");
            } finally {
                setLoadingYearStatusSuccess(false);
            }
        },
        [
            findYearStatusSuccessByCardNumber,
            setLoadingYearStatusSuccess,
            setLoadingYearStatusSuccess,
        ],
    );

    const debouncedFetchYearlySuccess = debounce(fetchYearlySuccess, 300);

    return {
        debouncedFetchYearlySuccess,
        loadingYearStatusSuccess,
        currentYearSuccessData,
        previousYearSuccessData,
        currentYearSuccess,
        previousYearSucces,
        yearPercentageSuccessChange,
        findYearStatusSuccessByCardNumber
    }



}