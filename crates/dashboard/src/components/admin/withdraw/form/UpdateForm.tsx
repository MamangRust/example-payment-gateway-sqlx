import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
  UpdateWithdrawFormValues,
  updateWithdrawRequestSchema,
} from "@/schemas";
import { zodResolver } from "@hookform/resolvers/zod";
import { useForm, Controller } from "react-hook-form";
import AsyncSelect from "react-select/async";
import { forwardRef, useCallback, useEffect, useState } from "react";
import { WithdrawUpdateFormProps } from "@/types/form";
import { FindAllCard, FindByCardNumber } from "@/types/domain/request";
import { debounce } from "@/helpers/debounce";
import useCardStore from "@/store/card/card";
import { useToast } from "@/hooks/use-toast";

interface CardOption {
  value: string;
  label: string;
}

const UpdateWithdrawForm = forwardRef<HTMLFormElement, WithdrawUpdateFormProps>(
  ({ onSubmit, defaultValues }, ref) => {
    const {
      register,
      handleSubmit,
      control,
      formState: { errors },
    } = useForm<UpdateWithdrawFormValues>({
      resolver: zodResolver(updateWithdrawRequestSchema),
      defaultValues: defaultValues || {
        card_number: {
          value: "",
          label: "",
        },
        withdraw_amount: 0,
        withdraw_time: new Date(),
      },
    });

    const [cardOptions, setCardOptions] = useState<CardOption[]>([]);

    const { toast } = useToast();

    const {
      cards,
      loadingGetCards,
      setLoadingGetCards,
      findAllCards,
      card,
      findByCardNumber,
    } = useCardStore();

    useEffect(() => {
      const fetchInitialCards = async () => {
        setLoadingGetCards(true);
        try {
          const searchReq: FindAllCard = {
            search: "",
            page: 1,
            page_size: 10,
            toast: toast,
          };
          await findAllCards(searchReq);

          if (cards) {
            const options = cards.map((card) => ({
              value: card.card_number,
              label: card.card_number,
            }));

            if (defaultValues?.card_number) {
              const isCardInList = options.some(
                (opt) => opt.value === defaultValues.card_number.value,
              );
              if (!isCardInList) {
                const req: FindByCardNumber = {
                  toast,
                  cardNumber: defaultValues.card_number.value,
                };

                await findByCardNumber(req);
                if (card) {
                  options.unshift({
                    value: card.card_number,
                    label: card.card_number,
                  });
                  console.log("test card", card);
                  console.log("test card option", options);
                }
              }
            }

            setCardOptions(options);
          }
        } catch (error) {
          console.error("Failed to fetch initial cards:", error);
        } finally {
          setLoadingGetCards(false);
        }
      };

      fetchInitialCards();
    }, []);

    const loadOptionCards = useCallback(
      debounce(
        async (
          inputValue: string,
          callback: (options: CardOption[]) => void,
        ) => {
          setLoadingGetCards(true);
          try {
            const searchReq: FindAllCard = {
              search: inputValue,
              page: 1,
              page_size: 10,
              toast: toast,
            };
            await findAllCards(searchReq);

            if (cards) {
              const filteredOptions = cards
                .filter((card) =>
                  card.card_number
                    .toLowerCase()
                    .includes(inputValue.toLowerCase()),
                )
                .map((card) => ({
                  value: card.card_number,
                  label: card.card_number,
                }));

              callback(filteredOptions);
            }
          } catch (error) {
            console.error("Failed to fetch cards:", error);
            callback([]);
          } finally {
            setLoadingGetCards(false);
          }
        },
        500,
      ),
      [cards, findAllCards, toast],
    );

    return (
      <form ref={ref} onSubmit={handleSubmit(onSubmit)} className="space-y-4">
        <div>
          <Label htmlFor="cardNumber" className="text-sm font-medium">
            Card Number
          </Label>
          <Controller
            name="card_number"
            control={control}
            render={({ field }) => (
              <AsyncSelect
                key={cardOptions.length}
                {...field}
                loadOptions={loadOptionCards}
                defaultOptions={cardOptions}
                isLoading={loadingGetCards}
                placeholder="Select a card"
                className="mt-1"
                styles={{
                  control: (base) => ({
                    ...base,
                    borderColor: errors.card_number
                      ? "#ef4444"
                      : base.borderColor,
                    borderRadius: "0.375rem",
                    padding: "0.25rem",
                    boxShadow: "none",
                    "&:hover": {
                      borderColor: "#3b82f6",
                    },
                  }),
                  option: (base, { isFocused, isSelected }) => ({
                    ...base,
                    backgroundColor: isSelected
                      ? "#3b82f6"
                      : isFocused
                        ? "#bfdbfe"
                        : "white",
                    color: isSelected ? "white" : "black",
                    "&:hover": {
                      backgroundColor: "#bfdbfe",
                    },
                  }),
                  menu: (base) => ({
                    ...base,
                    borderRadius: "0.375rem",
                    boxShadow: "0 4px 6px -1px rgb(0 0 0 / 0.1)",
                  }),
                }}
                value={
                  cardOptions.find(
                    (option) =>
                      option.value ===
                      (typeof field.value === "object"
                        ? field.value?.value
                        : field.value),
                  ) || {
                    value: defaultValues?.card_number?.value || "",
                    label: defaultValues?.card_number?.label || "Select a card",
                  }
                }
                onChange={(selectedOption) => {
                  field.onChange(
                    selectedOption
                      ? {
                          value: selectedOption.value,
                          label: selectedOption.label,
                        }
                      : null,
                  );
                }}
              />
            )}
          />
          {errors.card_number && (
            <p className="text-red-500 text-sm mt-1">
              {errors.card_number.message}
            </p>
          )}
        </div>
        <div>
          <Label htmlFor="withdraw_amount" className="text-sm font-medium">
            Withdraw Amount
          </Label>
          <Input
            id="withdraw_amount"
            type="number"
            placeholder="Enter withdraw amount"
            className="mt-1"
            {...register("withdraw_amount", { valueAsNumber: true })}
          />
          {errors.withdraw_amount && (
            <p className="text-red-500 text-sm mt-1">
              {errors.withdraw_amount.message}
            </p>
          )}
        </div>
        <div>
          <Label htmlFor="withdraw_time" className="text-sm font-medium">
            Withdraw Time
          </Label>
          <Controller
            name="withdraw_time"
            control={control}
            render={({ field }) => {
              const dateValue = field.value
                ? new Date(field.value)
                : new Date();
              const value = dateValue.toISOString().slice(0, 16);

              return (
                <Input
                  id="withdraw_time"
                  type="datetime-local"
                  className="mt-1"
                  value={value}
                  onChange={(e) => {
                    field.onChange(new Date(e.target.value));
                  }}
                />
              );
            }}
          />
          {errors.withdraw_time && (
            <p className="text-red-500 text-sm mt-1">
              {errors.withdraw_time.message}
            </p>
          )}
        </div>
      </form>
    );
  },
);

UpdateWithdrawForm.displayName = "UpdateWithdrawForm";

export default UpdateWithdrawForm;
