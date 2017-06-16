class MorrisseyRequestsController < ApplicationController

  def index
    @text = params[:text] || "Hi. I am the Morrissey bot"
    @query = params[:query] || "Say something"
  end

  def create
    @query = params[:text]
    @text = MorrisseyBot.translate(params[:text])
    render :index
  end

end
