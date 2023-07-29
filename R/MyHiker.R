#' @export
print.MyHiker = function(x, ...) {
  x$print()  
}

#' @export
MyHiker.DollarNames = function(self, pattern = "") {
  paste0(ls(helloextendr::MyHiker,pattern = pattern),"()")
}

