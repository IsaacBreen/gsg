%{
#include <stdio.h>
%}

%define lr.type ielr

%define lr.default-reduction accepting

%glr-parser

%token ATEQUAL GREATEREQUAL DEDENT TYPE YIELD _NAME TILDE RAISE RPAR RETURN DEL CIRCUMFLEXEQUAL FROM NUMBER TRUE RIGHTSHIFTEQUAL STAR ELLIPSIS DOUBLESLASH TRY LAMBDA NEWLINE AMPEREQUAL TYPE_COMMENT FINALLY INDENT RBRACE FSTRING_START PLUSEQUAL NOTEQUAL SEMI DEF LPAR PERCENTEQUAL IMPORT LSQB STRING CASE FSTRING_END MINUS ASYNC NONE DOUBLESTAR OR COMMA LESS LESSEQUAL ENDMARKER IN STAREQUAL DOUBLESLASHEQUAL ELSE AMPER GREATER AT CONTINUE GLOBAL VBAR RSQB LEFTSHIFTEQUAL ELIF EXCLAMATION MATCH COLONEQUAL EQEQUAL WITH SLASHEQUAL IS AS PERCENT UNDERSCORE COLON CLASS DOUBLESTAREQUAL AND IF BREAK EQUAL VBAREQUAL RARROW PLUS WHILE DOT ASSERT NONLOCAL FALSE PASS LEFTSHIFT MINEQUAL FSTRING_MIDDLE LBRACE NOT FOR EXCEPT AWAIT CIRCUMFLEX RIGHTSHIFT SLASH

%%
file: file__opt_248 ENDMARKER;

interactive: statement_newline;

eval: expressions eval__repeat0_247 ENDMARKER;

func_type: LPAR func_type__opt_247 RPAR RARROW expression func_type__repeat0_248 ENDMARKER;

statements: statements__repeat1_248;

statement: compound_stmt
    | simple_stmts;

statement_newline: compound_stmt NEWLINE
    | simple_stmts
    | NEWLINE
    | ENDMARKER;

simple_stmts: simple_stmt  NEWLINE
    | simple_stmts__gather_246 simple_stmts__opt_247 NEWLINE;

simple_stmt: assignment
    |  type_alias
    | star_expressions
    |  return_stmt
    |  import_stmt
    |  raise_stmt
    | PASS
    |  del_stmt
    |  yield_stmt
    |  assert_stmt
    | BREAK
    | CONTINUE
    |  global_stmt
    |  nonlocal_stmt;

compound_stmt:  function_def
    |  if_stmt
    |  class_def
    |  with_stmt
    |  for_stmt
    |  try_stmt
    |  while_stmt
    | match_stmt;

assignment: NAME COLON expression assignment__opt_245
    | assignment__group_246 COLON expression assignment__opt_247
    | assignment__repeat1_248 assignment__group_249  assignment__opt_250
    | single_target augassign  assignment__group_251;

annotated_rhs: yield_expr
    | star_expressions;

augassign: PLUSEQUAL
    | MINEQUAL
    | STAREQUAL
    | ATEQUAL
    | SLASHEQUAL
    | PERCENTEQUAL
    | AMPEREQUAL
    | VBAREQUAL
    | CIRCUMFLEXEQUAL
    | LEFTSHIFTEQUAL
    | RIGHTSHIFTEQUAL
    | DOUBLESTAREQUAL
    | DOUBLESLASHEQUAL;

return_stmt: RETURN return_stmt__opt_249;

raise_stmt: RAISE expression raise_stmt__opt_249
    | RAISE;

global_stmt: GLOBAL global_stmt__gather_249;

nonlocal_stmt: NONLOCAL nonlocal_stmt__gather_249;

del_stmt: DEL del_targets ;

yield_stmt: yield_expr;

assert_stmt: ASSERT expression assert_stmt__opt_247;

import_stmt: import_name
    | import_from;

import_name: IMPORT dotted_as_names;

import_from: FROM import_from__repeat0_245 dotted_name IMPORT import_from_targets
    | FROM import_from__repeat1_246 IMPORT import_from_targets;

import_from_targets: LPAR import_from_as_names import_from_targets__opt_246 RPAR
    | import_from_as_names 
    | STAR;

import_from_as_names: import_from_as_names__gather_246;

import_from_as_name: NAME import_from_as_name__opt_246;

dotted_as_names: dotted_as_names__gather_246;

dotted_as_name: dotted_name dotted_as_name__opt_246;

dotted_name: dotted_name DOT NAME
    | NAME;

block: NEWLINE INDENT statements DEDENT
    | simple_stmts;

decorators: decorators__repeat1_244;

class_def: decorators class_def_raw
    | class_def_raw;

class_def_raw: CLASS NAME class_def_raw__opt_243 class_def_raw__opt_244 COLON block;

function_def: decorators function_def_raw
    | function_def_raw;

function_def_raw: DEF NAME function_def_raw__opt_243 LPAR function_def_raw__opt_244 RPAR function_def_raw__opt_245 COLON function_def_raw__opt_246 block
    | ASYNC DEF NAME function_def_raw__opt_247 LPAR function_def_raw__opt_248 RPAR function_def_raw__opt_249 COLON function_def_raw__opt_250 block;

params: parameters;

parameters: slash_no_default parameters__repeat0_249 parameters__repeat0_250 parameters__opt_251
    | slash_with_default parameters__repeat0_252 parameters__opt_253
    | parameters__repeat1_254 parameters__repeat0_255 parameters__opt_256
    | parameters__repeat1_257 parameters__opt_258
    | star_etc;

slash_no_default: slash_no_default__repeat1_258 SLASH COMMA
    | slash_no_default__repeat1_259 SLASH ;

slash_with_default: slash_with_default__repeat0_259 slash_with_default__repeat1_260 SLASH COMMA
    | slash_with_default__repeat0_261 slash_with_default__repeat1_262 SLASH ;

star_etc: STAR param_no_default star_etc__repeat0_262 star_etc__opt_263
    | STAR param_no_default_star_annotation star_etc__repeat0_264 star_etc__opt_265
    | STAR COMMA star_etc__repeat1_266 star_etc__opt_267
    | kwds;

kwds: DOUBLESTAR param_no_default;

param_no_default: param COMMA param_no_default__opt_266
    | param param_no_default__opt_267 ;

param_no_default_star_annotation: param_star_annotation COMMA param_no_default_star_annotation__opt_267
    | param_star_annotation param_no_default_star_annotation__opt_268 ;

param_with_default: param default COMMA param_with_default__opt_268
    | param default param_with_default__opt_269 ;

param_maybe_default: param param_maybe_default__opt_269 COMMA param_maybe_default__opt_270
    | param param_maybe_default__opt_271 param_maybe_default__opt_272 ;

param: NAME param__opt_272;

param_star_annotation: NAME star_annotation;

annotation: COLON expression;

star_annotation: COLON star_expression;

default: EQUAL expression;

if_stmt: IF named_expression COLON block elif_stmt
    | IF named_expression COLON block if_stmt__opt_268;

elif_stmt: ELIF named_expression COLON block elif_stmt
    | ELIF named_expression COLON block elif_stmt__opt_268;

else_block: ELSE COLON block;

while_stmt: WHILE named_expression COLON block while_stmt__opt_267;

for_stmt: FOR star_targets IN  star_expressions COLON for_stmt__opt_267 block for_stmt__opt_268
    | ASYNC FOR star_targets IN  star_expressions COLON for_stmt__opt_269 block for_stmt__opt_270;

with_stmt: WITH LPAR with_stmt__gather_270 with_stmt__opt_271 RPAR COLON with_stmt__opt_272 block
    | WITH with_stmt__gather_273 COLON with_stmt__opt_274 block
    | ASYNC WITH LPAR with_stmt__gather_275 with_stmt__opt_276 RPAR COLON block
    | ASYNC WITH with_stmt__gather_277 COLON with_stmt__opt_278 block;

with_item: expression AS star_target 
    | expression;

try_stmt: TRY COLON block finally_block
    | TRY COLON block try_stmt__repeat1_277 try_stmt__opt_278 try_stmt__opt_279
    | TRY COLON block try_stmt__repeat1_280 try_stmt__opt_281 try_stmt__opt_282;

except_block: EXCEPT expression except_block__opt_282 COLON block
    | EXCEPT COLON block;

except_star_block: EXCEPT STAR expression except_star_block__opt_282 COLON block;

finally_block: FINALLY COLON block;

match_stmt: MATCH subject_expr COLON NEWLINE INDENT match_stmt__repeat1_281 DEDENT;

subject_expr: star_named_expression COMMA subject_expr__opt_281
    | named_expression;

case_block: CASE patterns case_block__opt_281 COLON block;

guard: IF named_expression;

patterns: open_sequence_pattern
    | pattern;

pattern: as_pattern
    | or_pattern;

as_pattern: or_pattern AS pattern_capture_target;

or_pattern: or_pattern__gather_277;

closed_pattern: literal_pattern
    | capture_pattern
    | wildcard_pattern
    | value_pattern
    | group_pattern
    | sequence_pattern
    | mapping_pattern
    | class_pattern;

literal_pattern: signed_number 
    | complex_number
    | strings
    | NONE
    | TRUE
    | FALSE;

literal_expr: signed_number 
    | complex_number
    | strings
    | NONE
    | TRUE
    | FALSE;

complex_number: signed_real_number PLUS imaginary_number
    | signed_real_number MINUS imaginary_number;

signed_number: NUMBER
    | MINUS NUMBER;

signed_real_number: real_number
    | MINUS real_number;

real_number: NUMBER;

imaginary_number: NUMBER;

capture_pattern: pattern_capture_target;

pattern_capture_target:  NAME ;

wildcard_pattern: UNDERSCORE;

value_pattern: attr ;

attr: name_or_attr DOT NAME;

name_or_attr: attr
    | NAME;

group_pattern: LPAR pattern RPAR;

sequence_pattern: LSQB sequence_pattern__opt_262 RSQB
    | LPAR sequence_pattern__opt_263 RPAR;

open_sequence_pattern: maybe_star_pattern COMMA open_sequence_pattern__opt_263;

maybe_sequence_pattern: maybe_sequence_pattern__gather_263 maybe_sequence_pattern__opt_264;

maybe_star_pattern: star_pattern
    | pattern;

star_pattern: STAR pattern_capture_target
    | STAR wildcard_pattern;

mapping_pattern: LBRACE RBRACE
    | LBRACE double_star_pattern mapping_pattern__opt_262 RBRACE
    | LBRACE items_pattern COMMA double_star_pattern mapping_pattern__opt_263 RBRACE
    | LBRACE items_pattern mapping_pattern__opt_264 RBRACE;

items_pattern: items_pattern__gather_264;

key_value_pattern: key_value_pattern__group_264 COLON pattern;

double_star_pattern: DOUBLESTAR pattern_capture_target;

class_pattern: name_or_attr LPAR RPAR
    | name_or_attr LPAR positional_patterns class_pattern__opt_263 RPAR
    | name_or_attr LPAR keyword_patterns class_pattern__opt_264 RPAR
    | name_or_attr LPAR positional_patterns COMMA keyword_patterns class_pattern__opt_265 RPAR;

positional_patterns: positional_patterns__gather_265;

keyword_patterns: keyword_patterns__gather_265;

keyword_pattern: NAME EQUAL pattern;

type_alias: TYPE NAME type_alias__opt_264 EQUAL expression;

type_params: LSQB type_param_seq RSQB;

type_param_seq: type_param_seq__gather_263 type_param_seq__opt_264;

type_param: NAME type_param__opt_264 type_param__opt_265
    | STAR NAME COLON expression
    | STAR NAME type_param__opt_266
    | DOUBLESTAR NAME COLON expression
    | DOUBLESTAR NAME type_param__opt_267;

type_param_bound: COLON expression;

type_param_default: EQUAL expression;

type_param_starred_default: EQUAL star_expression;

expressions: expression expressions__repeat1_264 expressions__opt_265
    | expression COMMA
    | expression;

expression: disjunction IF disjunction ELSE expression
    | disjunction
    | lambdef;

yield_expr: YIELD FROM expression
    | YIELD yield_expr__opt_264;

star_expressions: star_expression star_expressions__repeat1_264 star_expressions__opt_265
    | star_expression COMMA
    | star_expression;

star_expression: STAR bitwise_or
    | expression;

star_named_expressions: star_named_expressions__gather_264 star_named_expressions__opt_265;

star_named_expression: STAR bitwise_or
    | named_expression;

assignment_expression: NAME COLONEQUAL  expression;

named_expression: assignment_expression
    | expression ;

disjunction: conjunction disjunction__repeat1_262
    | conjunction;

conjunction: inversion conjunction__repeat1_262
    | inversion;

inversion: NOT inversion
    | comparison;

comparison: bitwise_or comparison__repeat1_261
    | bitwise_or;

compare_op_bitwise_or_pair: eq_bitwise_or
    | noteq_bitwise_or
    | lte_bitwise_or
    | lt_bitwise_or
    | gte_bitwise_or
    | gt_bitwise_or
    | notin_bitwise_or
    | in_bitwise_or
    | isnot_bitwise_or
    | is_bitwise_or;

eq_bitwise_or: EQEQUAL bitwise_or;

noteq_bitwise_or: noteq_bitwise_or__group_259 bitwise_or;

lte_bitwise_or: LESSEQUAL bitwise_or;

lt_bitwise_or: LESS bitwise_or;

gte_bitwise_or: GREATEREQUAL bitwise_or;

gt_bitwise_or: GREATER bitwise_or;

notin_bitwise_or: NOT IN bitwise_or;

in_bitwise_or: IN bitwise_or;

isnot_bitwise_or: IS NOT bitwise_or;

is_bitwise_or: IS bitwise_or;

bitwise_or: bitwise_or VBAR bitwise_xor
    | bitwise_xor;

bitwise_xor: bitwise_xor CIRCUMFLEX bitwise_and
    | bitwise_and;

bitwise_and: bitwise_and AMPER shift_expr
    | shift_expr;

shift_expr: shift_expr LEFTSHIFT sum
    | shift_expr RIGHTSHIFT sum
    | sum;

sum: sum PLUS term
    | sum MINUS term
    | term;

term: term STAR factor
    | term SLASH factor
    | term DOUBLESLASH factor
    | term PERCENT factor
    | term AT factor
    | factor;

factor: PLUS factor
    | MINUS factor
    | TILDE factor
    | power;

power: await_primary DOUBLESTAR factor
    | await_primary;

await_primary: AWAIT primary
    | primary;

primary: primary DOT NAME
    | primary genexp
    | primary LPAR primary__opt_242 RPAR
    | primary LSQB slices RSQB
    | atom;

slices: slice 
    | slices__gather_242 slices__opt_243;

slice: slice__opt_243 COLON slice__opt_244 slice__opt_245
    | named_expression;

atom: NAME
    | TRUE
    | FALSE
    | NONE
    |  strings
    | NUMBER
    |  atom__group_245
    |  atom__group_246
    |  atom__group_247
    | ELLIPSIS;

group: LPAR group__group_247 RPAR;

lambdef: LAMBDA lambdef__opt_247 COLON expression;

lambda_params: lambda_parameters;

lambda_parameters: lambda_slash_no_default lambda_parameters__repeat0_246 lambda_parameters__repeat0_247 lambda_parameters__opt_248
    | lambda_slash_with_default lambda_parameters__repeat0_249 lambda_parameters__opt_250
    | lambda_parameters__repeat1_251 lambda_parameters__repeat0_252 lambda_parameters__opt_253
    | lambda_parameters__repeat1_254 lambda_parameters__opt_255
    | lambda_star_etc;

lambda_slash_no_default: lambda_slash_no_default__repeat1_255 SLASH COMMA
    | lambda_slash_no_default__repeat1_256 SLASH ;

lambda_slash_with_default: lambda_slash_with_default__repeat0_256 lambda_slash_with_default__repeat1_257 SLASH COMMA
    | lambda_slash_with_default__repeat0_258 lambda_slash_with_default__repeat1_259 SLASH ;

lambda_star_etc: STAR lambda_param_no_default lambda_star_etc__repeat0_259 lambda_star_etc__opt_260
    | STAR COMMA lambda_star_etc__repeat1_261 lambda_star_etc__opt_262
    | lambda_kwds;

lambda_kwds: DOUBLESTAR lambda_param_no_default;

lambda_param_no_default: lambda_param COMMA
    | lambda_param ;

lambda_param_with_default: lambda_param default COMMA
    | lambda_param default ;

lambda_param_maybe_default: lambda_param lambda_param_maybe_default__opt_259 COMMA
    | lambda_param lambda_param_maybe_default__opt_260 ;

lambda_param: NAME;

fstring_middle: fstring_replacement_field
    | FSTRING_MIDDLE;

fstring_replacement_field: LBRACE annotated_rhs fstring_replacement_field__opt_258 fstring_replacement_field__opt_259 fstring_replacement_field__opt_260 RBRACE;

fstring_conversion: EXCLAMATION NAME;

fstring_full_format_spec: COLON fstring_full_format_spec__repeat0_259;

fstring_format_spec: FSTRING_MIDDLE
    | fstring_replacement_field;

fstring: FSTRING_START fstring__repeat0_258 FSTRING_END;

string: STRING;

strings: strings__repeat1_257;

list: LSQB list__opt_257 RSQB;

tuple: LPAR tuple__opt_257 RPAR;

set: LBRACE star_named_expressions RBRACE;

dict: LBRACE dict__opt_256 RBRACE;

double_starred_kvpairs: double_starred_kvpairs__gather_256 double_starred_kvpairs__opt_257;

double_starred_kvpair: DOUBLESTAR bitwise_or
    | kvpair;

kvpair: expression COLON expression;

for_if_clauses: for_if_clauses__repeat1_255;

for_if_clause: ASYNC FOR star_targets IN  disjunction for_if_clause__repeat0_255
    | FOR star_targets IN  disjunction for_if_clause__repeat0_256
    | for_if_clause__opt_257 FOR for_if_clause__group_258 ;

listcomp: LSQB named_expression for_if_clauses RSQB;

setcomp: LBRACE named_expression for_if_clauses RBRACE;

genexp: LPAR genexp__group_256 for_if_clauses RPAR;

dictcomp: LBRACE kvpair for_if_clauses RBRACE;

arguments: args arguments__opt_255 ;

args: args__gather_255 args__opt_256
    | kwargs;

kwargs: kwargs__gather_256 COMMA kwargs__gather_257
    | kwargs__gather_258
    | kwargs__gather_259;

starred_expression: STAR expression
    | STAR;

kwarg_or_starred: NAME EQUAL expression
    | starred_expression;

kwarg_or_double_starred: NAME EQUAL expression
    | DOUBLESTAR expression;

star_targets: star_target 
    | star_target star_targets__repeat0_256 star_targets__opt_257;

star_targets_list_seq: star_targets_list_seq__gather_257 star_targets_list_seq__opt_258;

star_targets_tuple_seq: star_target star_targets_tuple_seq__repeat1_258 star_targets_tuple_seq__opt_259
    | star_target COMMA;

star_target: STAR star_target__group_259
    | target_with_star_atom;

target_with_star_atom: t_primary DOT NAME 
    | t_primary LSQB slices RSQB 
    | star_atom;

star_atom: NAME
    | LPAR target_with_star_atom RPAR
    | LPAR star_atom__opt_258 RPAR
    | LSQB star_atom__opt_259 RSQB;

single_target: single_subscript_attribute_target
    | NAME
    | LPAR single_target RPAR;

single_subscript_attribute_target: t_primary DOT NAME 
    | t_primary LSQB slices RSQB ;

t_primary: t_primary DOT NAME 
    | t_primary LSQB slices RSQB 
    | t_primary genexp 
    | t_primary LPAR t_primary__opt_257 RPAR 
    | atom ;

t_lookahead: LPAR
    | LSQB
    | DOT;

del_targets: del_targets__gather_256 del_targets__opt_257;

del_target: t_primary DOT NAME 
    | t_primary LSQB slices RSQB 
    | del_t_atom;

del_t_atom: NAME
    | LPAR del_target RPAR
    | LPAR del_t_atom__opt_256 RPAR
    | LSQB del_t_atom__opt_257 RSQB;

type_expressions: type_expressions__gather_257 COMMA STAR expression COMMA DOUBLESTAR expression
    | type_expressions__gather_258 COMMA STAR expression
    | type_expressions__gather_259 COMMA DOUBLESTAR expression
    | STAR expression COMMA DOUBLESTAR expression
    | STAR expression
    | DOUBLESTAR expression
    | type_expressions__gather_260;

func_type_comment: NEWLINE TYPE_COMMENT 
    | TYPE_COMMENT;

expression_without_invalid: disjunction IF disjunction ELSE expression
    | disjunction
    | lambdef;

file__opt_248: file__opt_248_302
    | ;

eval__repeat0_247: eval__repeat0_247 NEWLINE
    | ;

func_type__opt_247: func_type__opt_247_301
    | ;

func_type__repeat0_248: func_type__repeat0_248 NEWLINE
    | ;

statements__repeat1_248: statements__repeat1_248 statement
    | statement;

simple_stmts__gather_246: simple_stmts__gather_246 SEMI simple_stmt
    | simple_stmt;

simple_stmts__opt_247: simple_stmts__opt_247_298
    | ;

assignment__opt_245: assignment__opt_245_298
    | ;

assignment__group_246: LPAR single_target RPAR
    | single_subscript_attribute_target;

assignment__opt_247: assignment__opt_247_297
    | ;

assignment__repeat1_248: assignment__repeat1_248 assignment__repeat1_248__group_297
    | assignment__repeat1_248__group_298;

assignment__group_249: yield_expr
    | star_expressions;

assignment__opt_250: assignment__opt_250_297
    | ;

assignment__group_251: yield_expr
    | star_expressions;

return_stmt__opt_249: return_stmt__opt_249_296
    | ;

raise_stmt__opt_249: raise_stmt__opt_249_296
    | ;

global_stmt__gather_249: global_stmt__gather_249 COMMA NAME
    | NAME;

nonlocal_stmt__gather_249: nonlocal_stmt__gather_249 COMMA NAME
    | NAME;

assert_stmt__opt_247: assert_stmt__opt_247_294
    | ;

import_from__repeat0_245: import_from__repeat0_245 import_from__repeat0_245__group_294
    | ;

import_from__repeat1_246: import_from__repeat1_246 import_from__repeat1_246__group_294
    | import_from__repeat1_246__group_295;

import_from_targets__opt_246: import_from_targets__opt_246_295
    | ;

import_from_as_names__gather_246: import_from_as_names__gather_246 COMMA import_from_as_name
    | import_from_as_name;

import_from_as_name__opt_246: import_from_as_name__opt_246_294
    | ;

dotted_as_names__gather_246: dotted_as_names__gather_246 COMMA dotted_as_name
    | dotted_as_name;

dotted_as_name__opt_246: dotted_as_name__opt_246_293
    | ;

decorators__repeat1_244: decorators__repeat1_244 decorators__repeat1_244__group_293
    | decorators__repeat1_244__group_294;

class_def_raw__opt_243: class_def_raw__opt_243_294
    | ;

class_def_raw__opt_244: class_def_raw__opt_244_294
    | ;

function_def_raw__opt_243: function_def_raw__opt_243_294
    | ;

function_def_raw__opt_244: function_def_raw__opt_244_294
    | ;

function_def_raw__opt_245: function_def_raw__opt_245_294
    | ;

function_def_raw__opt_246: function_def_raw__opt_246_294
    | ;

function_def_raw__opt_247: function_def_raw__opt_247_294
    | ;

function_def_raw__opt_248: function_def_raw__opt_248_294
    | ;

function_def_raw__opt_249: function_def_raw__opt_249_294
    | ;

function_def_raw__opt_250: function_def_raw__opt_250_294
    | ;

parameters__repeat0_249: parameters__repeat0_249 param_no_default
    | ;

parameters__repeat0_250: parameters__repeat0_250 param_with_default
    | ;

parameters__opt_251: parameters__opt_251_292
    | ;

parameters__repeat0_252: parameters__repeat0_252 param_with_default
    | ;

parameters__opt_253: parameters__opt_253_291
    | ;

parameters__repeat1_254: parameters__repeat1_254 param_no_default
    | param_no_default;

parameters__repeat0_255: parameters__repeat0_255 param_with_default
    | ;

parameters__opt_256: parameters__opt_256_289
    | ;

parameters__repeat1_257: parameters__repeat1_257 param_with_default
    | param_with_default;

parameters__opt_258: parameters__opt_258_288
    | ;

slash_no_default__repeat1_258: slash_no_default__repeat1_258 param_no_default
    | param_no_default;

slash_no_default__repeat1_259: slash_no_default__repeat1_259 param_no_default
    | param_no_default;

slash_with_default__repeat0_259: slash_with_default__repeat0_259 param_no_default
    | ;

slash_with_default__repeat1_260: slash_with_default__repeat1_260 param_with_default
    | param_with_default;

slash_with_default__repeat0_261: slash_with_default__repeat0_261 param_no_default
    | ;

slash_with_default__repeat1_262: slash_with_default__repeat1_262 param_with_default
    | param_with_default;

star_etc__repeat0_262: star_etc__repeat0_262 param_maybe_default
    | ;

star_etc__opt_263: star_etc__opt_263_281
    | ;

star_etc__repeat0_264: star_etc__repeat0_264 param_maybe_default
    | ;

star_etc__opt_265: star_etc__opt_265_280
    | ;

star_etc__repeat1_266: star_etc__repeat1_266 param_maybe_default
    | param_maybe_default;

star_etc__opt_267: star_etc__opt_267_279
    | ;

param_no_default__opt_266: TYPE_COMMENT
    | ;

param_no_default__opt_267: TYPE_COMMENT
    | ;

param_no_default_star_annotation__opt_267: TYPE_COMMENT
    | ;

param_no_default_star_annotation__opt_268: TYPE_COMMENT
    | ;

param_with_default__opt_268: TYPE_COMMENT
    | ;

param_with_default__opt_269: TYPE_COMMENT
    | ;

param_maybe_default__opt_269: default
    | ;

param_maybe_default__opt_270: TYPE_COMMENT
    | ;

param_maybe_default__opt_271: default
    | ;

param_maybe_default__opt_272: TYPE_COMMENT
    | ;

param__opt_272: annotation
    | ;

if_stmt__opt_268: if_stmt__opt_268_268
    | ;

elif_stmt__opt_268: elif_stmt__opt_268_268
    | ;

while_stmt__opt_267: while_stmt__opt_267_268
    | ;

for_stmt__opt_267: for_stmt__opt_267_268
    | ;

for_stmt__opt_268: for_stmt__opt_268_268
    | ;

for_stmt__opt_269: for_stmt__opt_269_268
    | ;

for_stmt__opt_270: for_stmt__opt_270_268
    | ;

with_stmt__gather_270: with_stmt__gather_270 COMMA with_item
    | with_item;

with_stmt__opt_271: COMMA
    | ;

with_stmt__opt_272: with_stmt__opt_272_266
    | ;

with_stmt__gather_273: with_stmt__gather_273 COMMA with_item
    | with_item;

with_stmt__opt_274: with_stmt__opt_274_265
    | ;

with_stmt__gather_275: with_stmt__gather_275 COMMA with_item
    | with_item;

with_stmt__opt_276: COMMA
    | ;

with_stmt__gather_277: with_stmt__gather_277 COMMA with_item
    | with_item;

with_stmt__opt_278: with_stmt__opt_278_262
    | ;

try_stmt__repeat1_277: try_stmt__repeat1_277 except_block
    | except_block;

try_stmt__opt_278: try_stmt__opt_278_261
    | ;

try_stmt__opt_279: try_stmt__opt_279_261
    | ;

try_stmt__repeat1_280: try_stmt__repeat1_280 except_star_block
    | except_star_block;

try_stmt__opt_281: try_stmt__opt_281_260
    | ;

try_stmt__opt_282: try_stmt__opt_282_260
    | ;

except_block__opt_282: except_block__opt_282_260
    | ;

except_star_block__opt_282: except_star_block__opt_282_260
    | ;

match_stmt__repeat1_281: match_stmt__repeat1_281 case_block
    | case_block;

subject_expr__opt_281: star_named_expressions
    | ;

case_block__opt_281: guard
    | ;

or_pattern__gather_277: or_pattern__gather_277 VBAR closed_pattern
    | closed_pattern;

sequence_pattern__opt_262: maybe_sequence_pattern
    | ;

sequence_pattern__opt_263: open_sequence_pattern
    | ;

open_sequence_pattern__opt_263: maybe_sequence_pattern
    | ;

maybe_sequence_pattern__gather_263: maybe_sequence_pattern__gather_263 COMMA maybe_star_pattern
    | maybe_star_pattern;

maybe_sequence_pattern__opt_264: COMMA
    | ;

mapping_pattern__opt_262: COMMA
    | ;

mapping_pattern__opt_263: COMMA
    | ;

mapping_pattern__opt_264: COMMA
    | ;

items_pattern__gather_264: items_pattern__gather_264 COMMA key_value_pattern
    | key_value_pattern;

key_value_pattern__group_264: literal_expr
    | attr;

class_pattern__opt_263: COMMA
    | ;

class_pattern__opt_264: COMMA
    | ;

class_pattern__opt_265: COMMA
    | ;

positional_patterns__gather_265: positional_patterns__gather_265 COMMA pattern
    | pattern;

keyword_patterns__gather_265: keyword_patterns__gather_265 COMMA keyword_pattern
    | keyword_pattern;

type_alias__opt_264: type_alias__opt_264_241
    | ;

type_param_seq__gather_263: type_param_seq__gather_263 COMMA type_param
    | type_param;

type_param_seq__opt_264: type_param_seq__opt_264_240
    | ;

type_param__opt_264: type_param__opt_264_240
    | ;

type_param__opt_265: type_param__opt_265_240
    | ;

type_param__opt_266: type_param__opt_266_240
    | ;

type_param__opt_267: type_param__opt_267_240
    | ;

expressions__repeat1_264: expressions__repeat1_264 expressions__repeat1_264__group_240
    | expressions__repeat1_264__group_241;

expressions__opt_265: expressions__opt_265_241
    | ;

yield_expr__opt_264: yield_expr__opt_264_241
    | ;

star_expressions__repeat1_264: star_expressions__repeat1_264 star_expressions__repeat1_264__group_241
    | star_expressions__repeat1_264__group_242;

star_expressions__opt_265: star_expressions__opt_265_242
    | ;

star_named_expressions__gather_264: star_named_expressions__gather_264 COMMA star_named_expression
    | star_named_expression;

star_named_expressions__opt_265: star_named_expressions__opt_265_241
    | ;

disjunction__repeat1_262: disjunction__repeat1_262 disjunction__repeat1_262__group_241
    | disjunction__repeat1_262__group_242;

conjunction__repeat1_262: conjunction__repeat1_262 conjunction__repeat1_262__group_242
    | conjunction__repeat1_262__group_243;

comparison__repeat1_261: comparison__repeat1_261 compare_op_bitwise_or_pair
    | compare_op_bitwise_or_pair;

noteq_bitwise_or__group_259: NOTEQUAL;

primary__opt_242: primary__opt_242_241
    | ;

slices__gather_242: slices__gather_242 COMMA slices__gather_242__group_241
    | slices__gather_242__group_242;

slices__opt_243: slices__opt_243_242
    | ;

slice__opt_243: slice__opt_243_242
    | ;

slice__opt_244: slice__opt_244_242
    | ;

slice__opt_245: slice__opt_245_242
    | ;

atom__group_245: tuple
    | group
    | genexp;

atom__group_246: list
    | listcomp;

atom__group_247: dict
    | set
    | dictcomp
    | setcomp;

group__group_247: yield_expr
    | named_expression;

lambdef__opt_247: lambdef__opt_247_238
    | ;

lambda_parameters__repeat0_246: lambda_parameters__repeat0_246 lambda_param_no_default
    | ;

lambda_parameters__repeat0_247: lambda_parameters__repeat0_247 lambda_param_with_default
    | ;

lambda_parameters__opt_248: lambda_parameters__opt_248_236
    | ;

lambda_parameters__repeat0_249: lambda_parameters__repeat0_249 lambda_param_with_default
    | ;

lambda_parameters__opt_250: lambda_parameters__opt_250_235
    | ;

lambda_parameters__repeat1_251: lambda_parameters__repeat1_251 lambda_param_no_default
    | lambda_param_no_default;

lambda_parameters__repeat0_252: lambda_parameters__repeat0_252 lambda_param_with_default
    | ;

lambda_parameters__opt_253: lambda_parameters__opt_253_233
    | ;

lambda_parameters__repeat1_254: lambda_parameters__repeat1_254 lambda_param_with_default
    | lambda_param_with_default;

lambda_parameters__opt_255: lambda_parameters__opt_255_232
    | ;

lambda_slash_no_default__repeat1_255: lambda_slash_no_default__repeat1_255 lambda_param_no_default
    | lambda_param_no_default;

lambda_slash_no_default__repeat1_256: lambda_slash_no_default__repeat1_256 lambda_param_no_default
    | lambda_param_no_default;

lambda_slash_with_default__repeat0_256: lambda_slash_with_default__repeat0_256 lambda_param_no_default
    | ;

lambda_slash_with_default__repeat1_257: lambda_slash_with_default__repeat1_257 lambda_param_with_default
    | lambda_param_with_default;

lambda_slash_with_default__repeat0_258: lambda_slash_with_default__repeat0_258 lambda_param_no_default
    | ;

lambda_slash_with_default__repeat1_259: lambda_slash_with_default__repeat1_259 lambda_param_with_default
    | lambda_param_with_default;

lambda_star_etc__repeat0_259: lambda_star_etc__repeat0_259 lambda_param_maybe_default
    | ;

lambda_star_etc__opt_260: lambda_star_etc__opt_260_225
    | ;

lambda_star_etc__repeat1_261: lambda_star_etc__repeat1_261 lambda_param_maybe_default
    | lambda_param_maybe_default;

lambda_star_etc__opt_262: lambda_star_etc__opt_262_224
    | ;

lambda_param_maybe_default__opt_259: default
    | ;

lambda_param_maybe_default__opt_260: default
    | ;

fstring_replacement_field__opt_258: EQUAL
    | ;

fstring_replacement_field__opt_259: fstring_replacement_field__opt_259_221
    | ;

fstring_replacement_field__opt_260: fstring_replacement_field__opt_260_221
    | ;

fstring_full_format_spec__repeat0_259: fstring_full_format_spec__repeat0_259 fstring_format_spec
    | ;

fstring__repeat0_258: fstring__repeat0_258 fstring_middle
    | ;

strings__repeat1_257: strings__repeat1_257 strings__repeat1_257__group_219
    | strings__repeat1_257__group_220;

list__opt_257: list__opt_257_220
    | ;

tuple__opt_257: tuple__opt_257_220
    | ;

dict__opt_256: dict__opt_256_220
    | ;

double_starred_kvpairs__gather_256: double_starred_kvpairs__gather_256 COMMA double_starred_kvpair
    | double_starred_kvpair;

double_starred_kvpairs__opt_257: double_starred_kvpairs__opt_257_219
    | ;

for_if_clauses__repeat1_255: for_if_clauses__repeat1_255 for_if_clause
    | for_if_clause;

for_if_clause__repeat0_255: for_if_clause__repeat0_255 for_if_clause__repeat0_255__group_218
    | ;

for_if_clause__repeat0_256: for_if_clause__repeat0_256 for_if_clause__repeat0_256__group_218
    | ;

for_if_clause__opt_257: ASYNC
    | ;

for_if_clause__group_258: bitwise_or for_if_clause__group_258__repeat0_217 for_if_clause__group_258__opt_218;

genexp__group_256: assignment_expression
    | expression ;

arguments__opt_255: arguments__opt_255_217
    | ;

args__gather_255: args__gather_255 COMMA args__gather_255__group_217
    | args__gather_255__group_218;

args__opt_256: args__opt_256_218
    | ;

kwargs__gather_256: kwargs__gather_256 COMMA kwarg_or_starred
    | kwarg_or_starred;

kwargs__gather_257: kwargs__gather_257 COMMA kwarg_or_double_starred
    | kwarg_or_double_starred;

kwargs__gather_258: kwargs__gather_258 COMMA kwarg_or_starred
    | kwarg_or_starred;

kwargs__gather_259: kwargs__gather_259 COMMA kwarg_or_double_starred
    | kwarg_or_double_starred;

star_targets__repeat0_256: star_targets__repeat0_256 star_targets__repeat0_256__group_214
    | ;

star_targets__opt_257: star_targets__opt_257_214
    | ;

star_targets_list_seq__gather_257: star_targets_list_seq__gather_257 COMMA star_target
    | star_target;

star_targets_list_seq__opt_258: star_targets_list_seq__opt_258_213
    | ;

star_targets_tuple_seq__repeat1_258: star_targets_tuple_seq__repeat1_258 star_targets_tuple_seq__repeat1_258__group_213
    | star_targets_tuple_seq__repeat1_258__group_214;

star_targets_tuple_seq__opt_259: star_targets_tuple_seq__opt_259_214
    | ;

star_target__group_259:  star_target;

star_atom__opt_258: star_atom__opt_258_213
    | ;

star_atom__opt_259: star_atom__opt_259_213
    | ;

t_primary__opt_257: t_primary__opt_257_213
    | ;

del_targets__gather_256: del_targets__gather_256 COMMA del_target
    | del_target;

del_targets__opt_257: del_targets__opt_257_212
    | ;

del_t_atom__opt_256: del_t_atom__opt_256_212
    | ;

del_t_atom__opt_257: del_t_atom__opt_257_212
    | ;

type_expressions__gather_257: type_expressions__gather_257 COMMA expression
    | expression;

type_expressions__gather_258: type_expressions__gather_258 COMMA expression
    | expression;

type_expressions__gather_259: type_expressions__gather_259 COMMA expression
    | expression;

type_expressions__gather_260: type_expressions__gather_260 COMMA expression
    | expression;

file__opt_248_302: statements;

func_type__opt_247_301: type_expressions;

simple_stmts__opt_247_298: SEMI;

assignment__opt_245_298: EQUAL annotated_rhs;

assignment__opt_247_297: EQUAL annotated_rhs;

assignment__repeat1_248__group_297: star_targets EQUAL;

assignment__repeat1_248__group_298: star_targets EQUAL;

assignment__opt_250_297: TYPE_COMMENT;

return_stmt__opt_249_296: star_expressions;

raise_stmt__opt_249_296: FROM expression;

assert_stmt__opt_247_294: COMMA expression;

import_from__repeat0_245__group_294: DOT
    | ELLIPSIS;

import_from__repeat1_246__group_294: DOT
    | ELLIPSIS;

import_from__repeat1_246__group_295: DOT
    | ELLIPSIS;

import_from_targets__opt_246_295: COMMA;

import_from_as_name__opt_246_294: AS NAME;

dotted_as_name__opt_246_293: AS NAME;

decorators__repeat1_244__group_293: AT named_expression NEWLINE;

decorators__repeat1_244__group_294: AT named_expression NEWLINE;

class_def_raw__opt_243_294: type_params;

class_def_raw__opt_244_294: LPAR class_def_raw__opt_244_294__opt_135 RPAR;

function_def_raw__opt_243_294: type_params;

function_def_raw__opt_244_294: params;

function_def_raw__opt_245_294: RARROW expression;

function_def_raw__opt_246_294: func_type_comment;

function_def_raw__opt_247_294: type_params;

function_def_raw__opt_248_294: params;

function_def_raw__opt_249_294: RARROW expression;

function_def_raw__opt_250_294: func_type_comment;

parameters__opt_251_292: star_etc;

parameters__opt_253_291: star_etc;

parameters__opt_256_289: star_etc;

parameters__opt_258_288: star_etc;

star_etc__opt_263_281: kwds;

star_etc__opt_265_280: kwds;

star_etc__opt_267_279: kwds;

if_stmt__opt_268_268: else_block;

elif_stmt__opt_268_268: else_block;

while_stmt__opt_267_268: else_block;

for_stmt__opt_267_268: TYPE_COMMENT;

for_stmt__opt_268_268: else_block;

for_stmt__opt_269_268: TYPE_COMMENT;

for_stmt__opt_270_268: else_block;

with_stmt__opt_272_266: TYPE_COMMENT;

with_stmt__opt_274_265: TYPE_COMMENT;

with_stmt__opt_278_262: TYPE_COMMENT;

try_stmt__opt_278_261: else_block;

try_stmt__opt_279_261: finally_block;

try_stmt__opt_281_260: else_block;

try_stmt__opt_282_260: finally_block;

except_block__opt_282_260: AS NAME;

except_star_block__opt_282_260: AS NAME;

type_alias__opt_264_241: type_params;

type_param_seq__opt_264_240: COMMA;

type_param__opt_264_240: type_param_bound;

type_param__opt_265_240: type_param_default;

type_param__opt_266_240: type_param_starred_default;

type_param__opt_267_240: type_param_default;

expressions__repeat1_264__group_240: COMMA expression;

expressions__repeat1_264__group_241: COMMA expression;

expressions__opt_265_241: COMMA;

yield_expr__opt_264_241: star_expressions;

star_expressions__repeat1_264__group_241: COMMA star_expression;

star_expressions__repeat1_264__group_242: COMMA star_expression;

star_expressions__opt_265_242: COMMA;

star_named_expressions__opt_265_241: COMMA;

disjunction__repeat1_262__group_241: OR conjunction;

disjunction__repeat1_262__group_242: OR conjunction;

conjunction__repeat1_262__group_242: AND inversion;

conjunction__repeat1_262__group_243: AND inversion;

primary__opt_242_241: arguments;

slices__gather_242__group_241: slice
    | starred_expression;

slices__gather_242__group_242: slice
    | starred_expression;

slices__opt_243_242: COMMA;

slice__opt_243_242: expression;

slice__opt_244_242: expression;

slice__opt_245_242: COLON slice__opt_245_242__opt_80;

lambdef__opt_247_238: lambda_params;

lambda_parameters__opt_248_236: lambda_star_etc;

lambda_parameters__opt_250_235: lambda_star_etc;

lambda_parameters__opt_253_233: lambda_star_etc;

lambda_parameters__opt_255_232: lambda_star_etc;

lambda_star_etc__opt_260_225: lambda_kwds;

lambda_star_etc__opt_262_224: lambda_kwds;

fstring_replacement_field__opt_259_221: fstring_conversion;

fstring_replacement_field__opt_260_221: fstring_full_format_spec;

strings__repeat1_257__group_219: fstring
    | string;

strings__repeat1_257__group_220: fstring
    | string;

list__opt_257_220: star_named_expressions;

tuple__opt_257_220: star_named_expression COMMA tuple__opt_257_220__opt_68;

dict__opt_256_220: double_starred_kvpairs;

double_starred_kvpairs__opt_257_219: COMMA;

for_if_clause__repeat0_255__group_218: IF disjunction;

for_if_clause__repeat0_256__group_218: IF disjunction;

for_if_clause__group_258__repeat0_217: for_if_clause__group_258__repeat0_217 for_if_clause__group_258__repeat0_217__group_64
    | ;

for_if_clause__group_258__opt_218: for_if_clause__group_258__opt_218_64
    | ;

arguments__opt_255_217: COMMA;

args__gather_255__group_217: starred_expression
    | args__gather_255__group_217__group_63 ;

args__gather_255__group_218: starred_expression
    | args__gather_255__group_218__group_63 ;

args__opt_256_218: COMMA kwargs;

star_targets__repeat0_256__group_214: COMMA star_target;

star_targets__opt_257_214: COMMA;

star_targets_list_seq__opt_258_213: COMMA;

star_targets_tuple_seq__repeat1_258__group_213: COMMA star_target;

star_targets_tuple_seq__repeat1_258__group_214: COMMA star_target;

star_targets_tuple_seq__opt_259_214: COMMA;

star_atom__opt_258_213: star_targets_tuple_seq;

star_atom__opt_259_213: star_targets_list_seq;

t_primary__opt_257_213: arguments;

del_targets__opt_257_212: COMMA;

del_t_atom__opt_256_212: del_targets;

del_t_atom__opt_257_212: del_targets;

class_def_raw__opt_244_294__opt_135: class_def_raw__opt_244_294__opt_135_18
    | ;

slice__opt_245_242__opt_80: slice__opt_245_242__opt_80_18
    | ;

tuple__opt_257_220__opt_68: tuple__opt_257_220__opt_68_18
    | ;

for_if_clause__group_258__repeat0_217__group_64: COMMA bitwise_or;

for_if_clause__group_258__opt_218_64: COMMA;

args__gather_255__group_217__group_63: assignment_expression
    | expression ;

args__gather_255__group_218__group_63: assignment_expression
    | expression ;

class_def_raw__opt_244_294__opt_135_18: arguments;

slice__opt_245_242__opt_80_18: expression;

tuple__opt_257_220__opt_68_18: star_named_expressions;

NAME: _NAME
    | FALSE
    | NONE
    | TRUE
    | AND
    | AS
    | ASSERT
    | ASYNC
    | AWAIT
    | BREAK
    | CLASS
    | CONTINUE
    | DEF
    | DEL
    | ELIF
    | ELSE
    | EXCEPT
    | FINALLY
    | FOR
    | FROM
    | GLOBAL
    | IF
    | IMPORT
    | IN
    | IS
    | LAMBDA
    | NONLOCAL
    | NOT
    | OR
    | PASS
    | RAISE
    | RETURN
    | TRY
    | WHILE
    | WITH
    | YIELD
    | UNDERSCORE
    | CASE
    | MATCH
    | TYPE;

%%

